use oxideterm_i18n::I18n;
use oxideterm_terminal::{TerminalNotification, TerminalNotificationSource};

use super::{WorkspaceNotificationKind, WorkspaceNotificationSeverity};

/// Notification-center title for a payload that carries no title of its own.
const NOTIFICATION_TITLE_KEY: &str = "event_log.terminal_notification_title";
/// Notification-center title for a bell, whose protocol carries no text at all.
const BELL_TITLE_KEY: &str = "event_log.terminal_bell_title";

/// A pane-owned terminal notification resolved into workspace-owned presentation.
pub(in crate::workspace) struct PlannedTerminalNotification {
    pub kind: WorkspaceNotificationKind,
    pub severity: WorkspaceNotificationSeverity,
    pub title: String,
    pub body: Option<String>,
    pub dedupe_key: Option<String>,
    /// Stable per-pane identity so repeated toasts replace each other instead of stacking.
    pub system_tag: String,
    pub show_system_notification: bool,
}

/// Resolve one pane notification for the notification center, or `None` when it
/// must not reach the user.
///
/// Only a bell from the pane the user is actually reading stays a flash, and a
/// backgrounded window is never being read even when its tab is active. Every
/// other notification reaches the notification center, and `window_active`
/// decides whether a system toast is posted as well.
pub(in crate::workspace) fn plan_terminal_notification(
    i18n: &I18n,
    notification: TerminalNotification,
    pane_label: Option<String>,
    pane_key: &str,
    pane_focused: bool,
    window_active: bool,
) -> Option<PlannedTerminalNotification> {
    let TerminalNotification {
        source,
        title,
        body,
    } = notification;
    let (title, body, dedupe_key) = match source {
        TerminalNotificationSource::Bell => {
            // The flash already covers a bell while the user is looking at the window.
            if pane_focused && window_active {
                return None;
            }
            // Programs repeat bells, so collapse them per pane; otherwise a chatty
            // program would fill the list with identical entries.
            (
                i18n.t(BELL_TITLE_KEY),
                pane_label,
                Some(format!("terminal-bell:{pane_key}")),
            )
        }
        _ => {
            // A payload without a title carries a single message, which becomes the
            // headline; the fallback covers payloads that carry no text at all.
            let (title, payload_body) = match title {
                Some(title) => (Some(title), body),
                None => (body, None),
            };
            let title = title.unwrap_or_else(|| i18n.t(NOTIFICATION_TITLE_KEY));
            (title, payload_body.or(pane_label), None)
        }
    };

    Some(PlannedTerminalNotification {
        kind: WorkspaceNotificationKind::Agent,
        severity: WorkspaceNotificationSeverity::Info,
        title,
        body,
        dedupe_key,
        system_tag: format!("terminal-notification:{pane_key}"),
        show_system_notification: !window_active,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxideterm_i18n::Locale;
    use oxideterm_terminal::TerminalNotificationSource;

    fn i18n() -> I18n {
        I18n::new(Locale::En)
    }

    fn osc9(message: &str) -> TerminalNotification {
        TerminalNotification {
            source: TerminalNotificationSource::Osc9,
            title: None,
            body: Some(message.to_string()),
        }
    }

    #[test]
    fn osc_message_keeps_its_wording_and_names_the_pane() {
        let plan = plan_terminal_notification(
            &i18n(),
            osc9("Turn complete \u{b7} demo"),
            Some("demo - grok".to_string()),
            "7",
            false,
            true,
        )
        .expect("an explicit notification is always recorded");

        assert_eq!(plan.title, "Turn complete \u{b7} demo");
        assert_eq!(plan.body.as_deref(), Some("demo - grok"));
        assert_eq!(plan.dedupe_key, None);
        assert!(
            !plan.show_system_notification,
            "an active window keeps the notification in-app"
        );
    }

    #[test]
    fn osc_body_without_title_uses_the_localized_fallback() {
        let plan = plan_terminal_notification(
            &i18n(),
            TerminalNotification {
                source: TerminalNotificationSource::Osc99,
                title: None,
                body: None,
            },
            None,
            "7",
            false,
            false,
        )
        .expect("fallback title");

        assert_eq!(plan.title, i18n().t(NOTIFICATION_TITLE_KEY));
        assert_eq!(plan.body, None);
        assert!(plan.show_system_notification);
    }

    #[test]
    fn osc777_title_and_body_are_preserved() {
        let plan = plan_terminal_notification(
            &i18n(),
            TerminalNotification {
                source: TerminalNotificationSource::Osc777,
                title: Some("Grok".to_string()),
                body: Some("Turn complete".to_string()),
            },
            Some("demo - grok".to_string()),
            "7",
            false,
            true,
        )
        .expect("osc777 payload");

        assert_eq!(plan.title, "Grok");
        assert_eq!(plan.body.as_deref(), Some("Turn complete"));
    }

    #[test]
    fn bell_from_the_focused_pane_stays_a_flash_while_the_window_is_active() {
        let focused = plan_terminal_notification(
            &i18n(),
            TerminalNotification::bell(),
            Some("demo - grok".to_string()),
            "7",
            true,
            true,
        );

        assert!(focused.is_none());
    }

    #[test]
    fn bell_from_the_active_tab_notifies_while_the_window_is_backgrounded() {
        let plan = plan_terminal_notification(
            &i18n(),
            TerminalNotification::bell(),
            Some("demo - grok".to_string()),
            "7",
            true,
            false,
        )
        .expect("a backgrounded window is not being read, even on the active tab");

        assert!(plan.show_system_notification);
        assert_eq!(plan.dedupe_key.as_deref(), Some("terminal-bell:7"));
    }

    #[test]
    fn bell_from_a_background_pane_is_deduped_per_pane() {
        let plan = plan_terminal_notification(
            &i18n(),
            TerminalNotification::bell(),
            Some("demo - grok".to_string()),
            "7",
            false,
            false,
        )
        .expect("background bell");

        assert_eq!(plan.title, i18n().t(BELL_TITLE_KEY));
        assert_eq!(plan.body.as_deref(), Some("demo - grok"));
        assert_eq!(plan.dedupe_key.as_deref(), Some("terminal-bell:7"));
        assert!(plan.show_system_notification);
        assert!(plan.system_tag.ends_with(":7"));
    }
}
