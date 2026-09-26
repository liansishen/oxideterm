// Copyright (C) 2026 AnalyseDeCircuit
// SPDX-License-Identifier: GPL-3.0-only

use oxideterm_settings::UpdateChannel;

pub const STABLE_UPDATE_ENDPOINT: &str =
    "https://github.com/AnalyseDeCircuit/oxideterm/releases/latest/download/latest.json";
pub const BETA_UPDATE_ENDPOINT: &str =
    "https://github.com/AnalyseDeCircuit/oxideterm/releases/download/updater-beta/latest.json";

pub fn normalize_update_repository(input: &str) -> Option<String> {
    let input = input.trim();
    let repository = input.strip_prefix("https://github.com/").unwrap_or(input);
    if repository.contains(['@', '?', '#', ':']) || repository.contains('\\') {
        return None;
    }
    let repository = repository.strip_suffix(".git").unwrap_or(repository);
    let mut parts = repository.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    if parts.next().is_some() || !valid_repository_part(owner) || !valid_repository_part(repo) {
        return None;
    }
    Some(format!("{owner}/{repo}"))
}

fn valid_repository_part(part: &str) -> bool {
    !part.is_empty()
        && part != "."
        && part != ".."
        && part.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.'
        })
}

#[cfg(test)]
mod tests {
    use super::normalize_update_repository;

    #[test]
    fn normalizes_github_repository_forms_and_rejects_unsafe_urls() {
        for (input, expected) in [
            ("owner/repo", "owner/repo"),
            ("https://github.com/owner/repo", "owner/repo"),
            ("https://github.com/owner/repo.git", "owner/repo"),
        ] {
            assert_eq!(
                normalize_update_repository(input).as_deref(),
                Some(expected)
            );
        }
        for input in [
            "https://user@github.com/owner/repo",
            "https://github.com/owner/repo?x=1",
            "https://github.com/owner/repo#tag",
            "https://github.com/owner/repo/tree/main",
            "http://github.com/owner/repo",
            "owner/repo/extra",
            "owner/",
        ] {
            assert_eq!(normalize_update_repository(input), None, "{input}");
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UpdateEndpoint {
    pub channel: UpdateChannel,
    pub url: &'static str,
}

pub fn endpoint_for_channel(
    channel: UpdateChannel,
) -> Result<UpdateEndpoint, crate::NativeUpdateError> {
    let url = match channel {
        UpdateChannel::Stable => STABLE_UPDATE_ENDPOINT,
        UpdateChannel::Beta => BETA_UPDATE_ENDPOINT,
        UpdateChannel::Custom => {
            return Err(crate::NativeUpdateError::General(
                "custom update channel requires a repository and public key".to_string(),
            ));
        }
    };
    Ok(UpdateEndpoint { channel, url })
}
