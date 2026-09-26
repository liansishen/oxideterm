const POST_CONNECT_COMMAND_MAX_BYTES: usize = 8192;

pub(crate) fn normalize_post_connect_command(
    command: Option<&str>,
) -> Result<Option<zeroize::Zeroizing<String>>, String> {
    let Some(command) = command.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };

    // Each logical line is submitted as an Enter key in the interactive shell.
    let mut normalized = zeroize::Zeroizing::new(String::with_capacity(command.len() + 1));
    let mut chars = command.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\r' && chars.peek() == Some(&'\n') {
            chars.next();
        }
        normalized.push(if ch == '\n' { '\r' } else { ch });
    }
    if !normalized.ends_with('\r') {
        normalized.push('\r');
    }

    if normalized.len() > POST_CONNECT_COMMAND_MAX_BYTES {
        return Err(format!(
            "Post-connect command is too long (max {} bytes)",
            POST_CONNECT_COMMAND_MAX_BYTES
        ));
    }
    Ok(Some(normalized))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_connect_command_normalizes_enter_keys_and_empty_values() {
        for (input, expected) in [
            (Some("  cd /srv/app  "), Some("cd /srv/app\r")),
            (Some("cd /srv/app\nls"), Some("cd /srv/app\rls\r")),
            (Some("pwd\r\nls\rwhoami\n"), Some("pwd\rls\rwhoami\r")),
            (Some(" \r\n "), None),
            (None, None),
        ] {
            let normalized = normalize_post_connect_command(input).unwrap();
            assert_eq!(normalized.as_ref().map(|value| value.as_str()), expected);
        }
    }

    #[test]
    fn post_connect_command_limit_includes_enter_and_counts_utf8_bytes() {
        let accepted = "a".repeat(POST_CONNECT_COMMAND_MAX_BYTES - 1);
        assert_eq!(
            normalize_post_connect_command(Some(&accepted))
                .unwrap()
                .unwrap()
                .as_str(),
            format!("{accepted}\r")
        );
        for oversized in [
            "a".repeat(POST_CONNECT_COMMAND_MAX_BYTES),
            "界".repeat(POST_CONNECT_COMMAND_MAX_BYTES / 3 + 1),
        ] {
            assert_eq!(
                normalize_post_connect_command(Some(&oversized)).unwrap_err(),
                "Post-connect command is too long (max 8192 bytes)"
            );
        }
    }
}
