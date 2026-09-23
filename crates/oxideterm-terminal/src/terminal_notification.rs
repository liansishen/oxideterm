use std::collections::VecDeque;

/// Longest retained text per field. Notification payloads come from the program
/// connected to the pane, so a broken or hostile sender must not be able to grow
/// pane-owned state without bound.
const MAX_TEXT_CHARS: usize = 512;

/// OSC 99 splits one notification across chunks. Keep a bounded number of
/// unfinished sequences so a sender that never terminates one cannot leak state.
const MAX_PENDING_KITTY_SEQUENCES: usize = 8;

/// Which notification protocol asked for the user's attention.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalNotificationSource {
    /// iTerm2, WezTerm, Warp: `ESC ] 9 ; message`.
    Osc9,
    /// Kitty: `ESC ] 99 ; metadata ; payload`, possibly split across chunks.
    Osc99,
    /// rxvt-unicode, Ghostty, VTE: `ESC ] 777 ; notify ; title ; body`.
    Osc777,
    /// C0 BEL: a generic attention request that carries no payload.
    Bell,
}

/// One attention request emitted by a program running inside the terminal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalNotification {
    pub source: TerminalNotificationSource,
    pub title: Option<String>,
    pub body: Option<String>,
}

impl TerminalNotification {
    /// A bell carries no text, so consumers label it themselves.
    pub fn bell() -> Self {
        Self {
            source: TerminalNotificationSource::Bell,
            title: None,
            body: None,
        }
    }
}

/// Which OSC 99 field a chunk carries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum KittyField {
    Title,
    Body,
}

#[derive(Default)]
struct PendingKittySequence {
    id: String,
    title: String,
    body: String,
    /// Field the previous chunk appended to. A following chunk for the same
    /// field extends it instead of replacing it.
    streaming: Option<KittyField>,
}

impl PendingKittySequence {
    fn into_notification(self) -> Option<TerminalNotification> {
        let title = trimmed(self.title);
        let body = trimmed(self.body);
        (title.is_some() || body.is_some()).then_some(TerminalNotification {
            source: TerminalNotificationSource::Osc99,
            title,
            body,
        })
    }
}

/// Parses the notification OSC protocols and reassembles chunked OSC 99 payloads.
#[derive(Default)]
pub(crate) struct TerminalNotificationTracker {
    pending_kitty: VecDeque<PendingKittySequence>,
}

impl TerminalNotificationTracker {
    /// Consume one `ESC ] <code> ; <data>` payload.
    ///
    /// Returns `true` when the sequence belongs to a notification protocol, so
    /// the caller consumes the raw bytes instead of forwarding them to the
    /// terminal emulator.
    pub(crate) fn observe_osc(
        &mut self,
        code: &str,
        data: &str,
        emit: &mut impl FnMut(TerminalNotification),
    ) -> bool {
        match code {
            "9" => {
                // `9;4;<state>;<percent>` reports task progress, which the emulator owns.
                if is_progress_report(data) {
                    return false;
                }
                if let Some(body) = sanitize(data) {
                    emit(TerminalNotification {
                        source: TerminalNotificationSource::Osc9,
                        title: None,
                        body: Some(body),
                    });
                }
                true
            }
            "99" => {
                self.observe_kitty(data, emit);
                true
            }
            "777" => match parse_osc777(data) {
                Some(notification) => {
                    emit(notification);
                    true
                }
                None => false,
            },
            _ => false,
        }
    }

    fn observe_kitty(&mut self, data: &str, emit: &mut impl FnMut(TerminalNotification)) {
        // `ESC ] 99 ; metadata ; payload`; the metadata may be empty.
        let (metadata, payload) = data.split_once(';').unwrap_or(("", data));
        let mut id: Option<&str> = None;
        let mut field = KittyField::Title;
        // Chunks announce more data with `d=0`; everything else completes the sequence.
        let mut complete = true;
        for pair in metadata.split(':') {
            let Some((key, value)) = pair.split_once('=') else {
                continue;
            };
            match key {
                "i" => id = Some(value),
                "p" => {
                    field = if value == "body" {
                        KittyField::Body
                    } else {
                        KittyField::Title
                    }
                }
                "d" => complete = value == "1",
                _ => {}
            }
        }

        let Some(id) = id else {
            // A single-shot payload. Kitty defaults an absent kind to the title.
            let Some(text) = sanitize(payload) else {
                return;
            };
            emit(match field {
                KittyField::Title => TerminalNotification {
                    source: TerminalNotificationSource::Osc99,
                    title: Some(text),
                    body: None,
                },
                KittyField::Body => TerminalNotification {
                    source: TerminalNotificationSource::Osc99,
                    title: None,
                    body: Some(text),
                },
            });
            return;
        };

        let index = match self.pending_kitty.iter().position(|entry| entry.id == id) {
            Some(index) => index,
            None => {
                self.reserve_kitty_slot(emit);
                self.pending_kitty.push_back(PendingKittySequence {
                    id: id.to_owned(),
                    ..PendingKittySequence::default()
                });
                self.pending_kitty.len() - 1
            }
        };

        {
            let entry = &mut self.pending_kitty[index];
            let streaming = entry.streaming == Some(field);
            let target = match field {
                KittyField::Title => &mut entry.title,
                KittyField::Body => &mut entry.body,
            };
            if streaming {
                append_capped(target, &sanitize_text(payload));
            } else {
                *target = sanitize_text(payload);
            }
            entry.streaming = (!complete).then_some(field);
        }

        if complete
            && let Some(entry) = self.pending_kitty.remove(index)
            && let Some(notification) = entry.into_notification()
        {
            emit(notification);
        }
    }

    /// Bounded retention: an unfinished sequence is reported rather than kept
    /// forever, so a sender that never completes one cannot stall the others.
    fn reserve_kitty_slot(&mut self, emit: &mut impl FnMut(TerminalNotification)) {
        if self.pending_kitty.len() < MAX_PENDING_KITTY_SEQUENCES {
            return;
        }
        if let Some(dropped) = self.pending_kitty.pop_front()
            && let Some(notification) = dropped.into_notification()
        {
            emit(notification);
        }
    }
}

/// `ESC ] 777 ; notify ; title [; body]`; other 777 subcommands stay untouched.
fn parse_osc777(data: &str) -> Option<TerminalNotification> {
    let (subcommand, rest) = data.split_once(';')?;
    if subcommand != "notify" {
        return None;
    }
    let (title, body) = match rest.split_once(';') {
        Some((title, body)) => (sanitize(title), sanitize(body)),
        None => (sanitize(rest), None),
    };
    (title.is_some() || body.is_some()).then_some(TerminalNotification {
        source: TerminalNotificationSource::Osc777,
        title,
        body,
    })
}

/// `ESC ] 9 ; 4 ; ...` reports task progress (ConEmu, Windows Terminal, Ghostty).
fn is_progress_report(data: &str) -> bool {
    data.starts_with("4;")
}

fn trimmed(text: String) -> Option<String> {
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

/// Drop control characters so a payload cannot forge further terminal output or
/// break the notification layout, and cap the length of untrusted text.
fn sanitize_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len().min(MAX_TEXT_CHARS));
    let mut chars = 0usize;
    let mut pending_space = false;
    for ch in text.chars() {
        if ch.is_control() {
            pending_space |= !out.is_empty();
            continue;
        }
        if chars >= MAX_TEXT_CHARS {
            break;
        }
        if pending_space {
            out.push(' ');
            chars += 1;
            pending_space = false;
        }
        out.push(ch);
        chars += 1;
    }
    out
}

/// Sanitized text that has actual content.
fn sanitize(text: &str) -> Option<String> {
    trimmed(sanitize_text(text))
}

fn append_capped(target: &mut String, text: &str) {
    let remaining = MAX_TEXT_CHARS.saturating_sub(target.chars().count());
    if remaining == 0 {
        return;
    }
    target.extend(text.chars().take(remaining));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(
        tracker: &mut TerminalNotificationTracker,
        code: &str,
        data: &str,
    ) -> (bool, Vec<TerminalNotification>) {
        let mut emitted = Vec::new();
        let handled =
            tracker.observe_osc(code, data, &mut |notification| emitted.push(notification));
        (handled, emitted)
    }

    #[test]
    fn osc9_message_is_the_body() {
        let mut tracker = TerminalNotificationTracker::default();
        let (handled, emitted) = collect(&mut tracker, "9", "Turn complete \u{b7} session");

        assert!(handled);
        assert_eq!(
            emitted,
            vec![TerminalNotification {
                source: TerminalNotificationSource::Osc9,
                title: None,
                body: Some("Turn complete \u{b7} session".to_string()),
            }]
        );
    }

    #[test]
    fn osc9_progress_is_not_a_notification() {
        let mut tracker = TerminalNotificationTracker::default();
        let (handled, emitted) = collect(&mut tracker, "9", "4;1;-1");

        assert!(!handled);
        assert!(emitted.is_empty());
    }

    #[test]
    fn osc9_empty_payload_is_consumed_without_a_notification() {
        let mut tracker = TerminalNotificationTracker::default();
        let (handled, emitted) = collect(&mut tracker, "9", "   ");

        assert!(handled);
        assert!(emitted.is_empty());
    }

    #[test]
    fn osc777_notify_splits_title_and_body() {
        let mut tracker = TerminalNotificationTracker::default();
        let (handled, emitted) = collect(&mut tracker, "777", "notify;Grok;Turn complete");

        assert!(handled);
        assert_eq!(emitted[0].source, TerminalNotificationSource::Osc777);
        assert_eq!(emitted[0].title.as_deref(), Some("Grok"));
        assert_eq!(emitted[0].body.as_deref(), Some("Turn complete"));
    }

    #[test]
    fn osc777_other_subcommands_stay_unhandled() {
        let mut tracker = TerminalNotificationTracker::default();
        let (handled, emitted) = collect(&mut tracker, "777", "other;payload");

        assert!(!handled);
        assert!(emitted.is_empty());
    }

    #[test]
    fn osc99_single_shot_is_a_title() {
        let mut tracker = TerminalNotificationTracker::default();
        let (handled, emitted) = collect(&mut tracker, "99", "i=grok;Turn complete");

        assert!(handled);
        assert_eq!(emitted.len(), 1);
        assert_eq!(emitted[0].title.as_deref(), Some("Turn complete"));
        assert_eq!(emitted[0].body, None);
    }

    #[test]
    fn osc99_chunks_merge_into_one_notification() {
        let mut tracker = TerminalNotificationTracker::default();
        let (_, first) = collect(&mut tracker, "99", "i=1:d=0;grok finished");
        let (_, second) = collect(&mut tracker, "99", "i=1:p=body;ws \u{b7} 1");

        assert!(
            first.is_empty(),
            "an unfinished sequence must not notify yet"
        );
        assert_eq!(second.len(), 1);
        assert_eq!(second[0].title.as_deref(), Some("grok finished"));
        assert_eq!(second[0].body.as_deref(), Some("ws \u{b7} 1"));
    }

    #[test]
    fn osc99_streamed_body_chunks_accumulate() {
        let mut tracker = TerminalNotificationTracker::default();
        collect(&mut tracker, "99", "i=7:d=0;title");
        collect(&mut tracker, "99", "i=7:p=body:d=0;one ");
        let (_, emitted) = collect(&mut tracker, "99", "i=7:p=body:d=1;two");

        assert_eq!(emitted.len(), 1);
        assert_eq!(emitted[0].body.as_deref(), Some("one two"));
    }

    #[test]
    fn osc99_unfinished_sequences_are_bounded() {
        let mut tracker = TerminalNotificationTracker::default();
        let mut emitted = Vec::new();
        for index in 0..=MAX_PENDING_KITTY_SEQUENCES {
            tracker.observe_osc(
                "99",
                &format!("i={index}:d=0;pending {index}"),
                &mut |notification| emitted.push(notification),
            );
        }

        assert_eq!(tracker.pending_kitty.len(), MAX_PENDING_KITTY_SEQUENCES);
        assert_eq!(
            emitted.len(),
            1,
            "the oldest unfinished sequence is reported"
        );
        assert_eq!(emitted[0].title.as_deref(), Some("pending 0"));
    }

    #[test]
    fn payloads_strip_controls_and_collapse_whitespace() {
        let mut tracker = TerminalNotificationTracker::default();
        let (_, emitted) = collect(&mut tracker, "9", "done\u{1b}]0;forged\u{9c}\nnext");

        assert_eq!(emitted[0].body.as_deref(), Some("done ]0;forged next"));
    }

    #[test]
    fn payload_length_is_capped() {
        let mut tracker = TerminalNotificationTracker::default();
        let (_, emitted) = collect(&mut tracker, "9", &"x".repeat(MAX_TEXT_CHARS * 4));

        let body = emitted[0].body.as_deref().expect("payload retained");
        assert_eq!(body.chars().count(), MAX_TEXT_CHARS);
    }

    #[test]
    fn bell_notification_has_no_payload() {
        let notification = TerminalNotification::bell();

        assert_eq!(notification.source, TerminalNotificationSource::Bell);
        assert_eq!(notification.title, None);
        assert_eq!(notification.body, None);
    }
}
