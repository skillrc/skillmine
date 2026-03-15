pub fn parse_github_ref(
    input: &str,
) -> Result<(String, Option<String>), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() < 2 {
        return Err(format!(
            "Invalid GitHub ref '{}'. Expected: owner/repo or owner/repo/path",
            input
        )
        .into());
    }

    let owner = parts[0];
    let repo = parts[1];

    if owner.is_empty() || repo.is_empty() {
        return Err("Owner and repo cannot be empty".into());
    }

    let path = if parts.len() > 2 {
        Some(parts[2..].join("/"))
    } else {
        None
    };

    Ok((format!("{}/{}", owner, repo), path))
}

#[allow(dead_code)]
pub fn build_github_url(repo: &str) -> String {
    format!("https://github.com/{}", repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_repo() {
        let (repo, path) = parse_github_ref("anthropic/skills").unwrap();
        assert_eq!(repo, "anthropic/skills");
        assert_eq!(path, None);
    }

    #[test]
    fn test_parse_with_path() {
        let (repo, path) = parse_github_ref("anthropic/skills/git-release").unwrap();
        assert_eq!(repo, "anthropic/skills");
        assert_eq!(path, Some("git-release".to_string()));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_github_ref("invalid").is_err());
        assert!(parse_github_ref("/repo").is_err());
        assert!(parse_github_ref("owner/").is_err());
    }

    #[test]
    fn test_build_url() {
        assert_eq!(
            build_github_url("anthropic/skills"),
            "https://github.com/anthropic/skills"
        );
    }
}
