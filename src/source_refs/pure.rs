pub fn github_repo_url(repo: &str) -> String {
    if repo.starts_with('/') || repo.starts_with("./") || repo.starts_with("../") {
        repo.to_string()
    } else {
        format!("https://github.com/{}", repo)
    }
}

pub fn reference_for_requested_ref(
    branch: &Option<String>,
    tag: &Option<String>,
    commit: &Option<String>,
) -> Option<String> {
    if let Some(commit) = commit {
        Some(commit.clone())
    } else if let Some(tag) = tag {
        Some(format!("tag:{}", tag))
    } else {
        branch.as_ref().map(|branch| format!("branch:{}", branch))
    }
}

#[allow(dead_code)]
pub fn checkout_ref_for_requested_ref(
    branch: &Option<String>,
    tag: &Option<String>,
    commit: &Option<String>,
) -> Option<String> {
    if let Some(commit) = commit {
        Some(commit.clone())
    } else if let Some(tag) = tag {
        Some(format!("refs/tags/{}", tag))
    } else {
        branch.as_ref().map(|branch| format!("origin/{}", branch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_helpers() {
        assert_eq!(
            reference_for_requested_ref(&Some("main".to_string()), &None, &None),
            Some("branch:main".to_string())
        );
        assert_eq!(
            reference_for_requested_ref(&None, &Some("v1.0".to_string()), &None),
            Some("tag:v1.0".to_string())
        );
        assert_eq!(
            checkout_ref_for_requested_ref(&Some("main".to_string()), &None, &None),
            Some("origin/main".to_string())
        );
        assert_eq!(
            github_repo_url("owner/repo"),
            "https://github.com/owner/repo"
        );
        assert_eq!(github_repo_url("/tmp/repo"), "/tmp/repo");
    }
}
