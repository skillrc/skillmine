use crate::config::SkillSource;
use crate::error::Result;
use crate::error::SkillmineError;
use crate::registry::pure::{github_repo_url, reference_for_requested_ref};
use git2::{build::RepoBuilder, FetchOptions, Oid, Repository};
use std::path::Path;

/// Git client for cloning and managing skill repositories
pub struct GitClient;

/// Resolved Git reference information
#[derive(Debug, Clone)]
pub struct ResolvedRef {
    #[allow(dead_code)]
    pub commit: String,
    pub tree_hash: String,
    #[allow(dead_code)]
    pub reference: String,
}

impl GitClient {
    pub fn has_resolvable_head(repo_path: &Path) -> bool {
        Self::resolve_local_head(repo_path).is_ok()
    }

    pub fn clone_and_resolve(
        source: &SkillSource,
        dest: &Path,
        shallow: bool,
    ) -> Result<ResolvedRef> {
        Self::clone_skill(source, dest, shallow)?;
        Self::resolve_source(source, dest)
    }

    pub fn resolve_source(source: &SkillSource, repo_path: &Path) -> Result<ResolvedRef> {
        match source {
            SkillSource::GitHub {
                path,
                branch,
                tag,
                commit,
                ..
            } => {
                let resolved = Self::resolve_local_head(repo_path)?;
                let tree_hash = Self::get_path_tree_hash(repo_path, path)?;

                let reference =
                    reference_for_requested_ref(branch, tag, commit).unwrap_or(resolved.reference);

                Ok(ResolvedRef {
                    commit: resolved.commit,
                    tree_hash,
                    reference,
                })
            }
            SkillSource::Local { .. } => Self::resolve_local_head(repo_path),
            SkillSource::Version(_) => Err(SkillmineError::Git(
                "Version-based resolution not yet implemented - use GitHub source".to_string(),
            )),
        }
    }

    pub fn resolve_local_head(repo_path: &Path) -> Result<ResolvedRef> {
        let repo = Repository::open(repo_path)
            .map_err(|e| SkillmineError::Registry(format!("Failed to open local repo: {}", e)))?;

        Self::get_resolved_ref(&repo, &None, &None, &None)
    }

    pub fn clone_skill(source: &SkillSource, dest: &Path, shallow: bool) -> Result<ResolvedRef> {
        match source {
            SkillSource::GitHub {
                repo,
                branch,
                tag,
                commit,
                ..
            } => {
                let url = github_repo_url(repo);

                let mut fetch_opts = FetchOptions::new();
                if shallow {
                    fetch_opts.depth(1);
                }

                let mut builder = RepoBuilder::new();
                builder.fetch_options(fetch_opts);

                if let Some(branch_name) = branch {
                    builder.branch(branch_name);
                }

                let repo = builder
                    .clone(&url, dest)
                    .map_err(|e| SkillmineError::Git(format!("Failed to clone {}: {}", url, e)))?;

                if let Some(tag_name) = tag {
                    Self::checkout_tag(&repo, tag_name)?;
                } else if let Some(commit_sha) = commit {
                    Self::checkout_commit(&repo, commit_sha)?;
                }

                Self::get_resolved_ref(&repo, branch, tag, commit)
            }
            SkillSource::Local { path: local_path } => {
                Self::resolve_local_head(Path::new(local_path))
            }
            SkillSource::Version(_) => Err(SkillmineError::Git(
                "Version-based resolution not yet implemented - use GitHub source".to_string(),
            )),
        }
    }

    fn checkout_commit(repo: &Repository, commit_sha: &str) -> Result<()> {
        let oid = Oid::from_str(commit_sha)
            .map_err(|e| SkillmineError::Git(format!("Invalid commit SHA: {}", e)))?;

        let commit = repo
            .find_commit(oid)
            .map_err(|e| SkillmineError::Git(format!("Commit not found: {}", e)))?;

        repo.checkout_tree(commit.as_object(), None)
            .map_err(|e| SkillmineError::Git(format!("Checkout failed: {}", e)))?;

        repo.set_head_detached(oid)
            .map_err(|e| SkillmineError::Git(format!("Failed to set HEAD: {}", e)))?;

        Ok(())
    }

    fn checkout_tag(repo: &Repository, tag_name: &str) -> Result<()> {
        let object = repo
            .revparse_single(&format!("refs/tags/{}", tag_name))
            .map_err(|e| SkillmineError::Git(format!("Tag not found: {}", e)))?;

        repo.checkout_tree(&object, None)
            .map_err(|e| SkillmineError::Git(format!("Checkout failed: {}", e)))?;

        repo.set_head_detached(object.id())
            .map_err(|e| SkillmineError::Git(format!("Failed to set HEAD: {}", e)))?;

        Ok(())
    }

    fn get_resolved_ref(
        repo: &Repository,
        branch: &Option<String>,
        tag: &Option<String>,
        commit: &Option<String>,
    ) -> Result<ResolvedRef> {
        let head = repo
            .head()
            .map_err(|e| SkillmineError::Git(format!("Failed to get HEAD: {}", e)))?;

        let head_commit = head
            .peel_to_commit()
            .map_err(|e| SkillmineError::Git(format!("Failed to peel to commit: {}", e)))?;

        let commit_sha = head_commit.id().to_string();
        let tree_hash = head_commit
            .tree()
            .map_err(|e| SkillmineError::Git(format!("Failed to get tree: {}", e)))?
            .id()
            .to_string();

        let reference = reference_for_requested_ref(branch, tag, commit)
            .unwrap_or_else(|| "default".to_string());

        Ok(ResolvedRef {
            commit: commit_sha,
            tree_hash,
            reference,
        })
    }

    #[allow(dead_code)]
    pub fn get_path_tree_hash(repo_path: &Path, subpath: &Option<String>) -> Result<String> {
        let repo = Repository::open(repo_path)
            .map_err(|e| SkillmineError::Git(format!("Failed to open repo: {}", e)))?;

        let head = repo
            .head()
            .map_err(|e| SkillmineError::Git(format!("Failed to get HEAD: {}", e)))?;

        let commit = head
            .peel_to_commit()
            .map_err(|e| SkillmineError::Git(format!("Failed to get commit: {}", e)))?;

        if let Some(path) = subpath {
            let tree = commit
                .tree()
                .map_err(|e| SkillmineError::Git(format!("Failed to get tree: {}", e)))?;

            let entry = tree
                .get_path(Path::new(path))
                .map_err(|e| SkillmineError::Git(format!("Path not found in repo: {}", e)))?;

            Ok(entry.id().to_string())
        } else {
            Ok(commit
                .tree()
                .map_err(|e| SkillmineError::Git(format!("Failed to get tree: {}", e)))?
                .id()
                .to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Repository, Signature};
    use std::fs;
    use tempfile::TempDir;

    fn init_test_repo() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();

        fs::create_dir_all(temp_dir.path().join("nested")).unwrap();
        fs::write(temp_dir.path().join("README.md"), "hello").unwrap();
        fs::write(temp_dir.path().join("nested").join("skill.md"), "nested").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("README.md")).unwrap();
        index.add_path(Path::new("nested/skill.md")).unwrap();
        index.write().unwrap();

        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = Signature::now("skillmine", "skillmine@example.com").unwrap();

        repo.commit(Some("HEAD"), &signature, &signature, "initial", &tree, &[])
            .unwrap();

        drop(tree);
        drop(repo);

        temp_dir
    }

    #[test]
    fn test_get_path_tree_hash_for_root_and_subpath() {
        let temp_dir = init_test_repo();

        let root_hash = GitClient::get_path_tree_hash(temp_dir.path(), &None).unwrap();
        let nested_hash =
            GitClient::get_path_tree_hash(temp_dir.path(), &Some("nested".to_string())).unwrap();

        assert_eq!(root_hash.len(), 40);
        assert_eq!(nested_hash.len(), 40);
        assert_ne!(root_hash, nested_hash);
    }

    #[test]
    fn test_clone_skill_from_local_repo() {
        let temp_dir = init_test_repo();
        let destination = TempDir::new().unwrap();
        let source = SkillSource::Local {
            path: temp_dir.path().to_string_lossy().to_string(),
        };

        let resolved = GitClient::clone_skill(&source, destination.path(), true).unwrap();

        assert_eq!(resolved.commit.len(), 40);
        assert_eq!(resolved.tree_hash.len(), 40);
        assert_eq!(resolved.reference, "default");
    }

    #[test]
    fn test_resolve_local_head() {
        let temp_dir = init_test_repo();
        let resolved = GitClient::resolve_local_head(temp_dir.path()).unwrap();

        assert_eq!(resolved.commit.len(), 40);
        assert_eq!(resolved.tree_hash.len(), 40);
        assert_eq!(resolved.reference, "default");
    }

    #[test]
    fn test_has_resolvable_head_detects_broken_repo() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        std::fs::create_dir_all(repo_path.join(".git/refs/heads")).unwrap();
        std::fs::write(repo_path.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        std::fs::write(
            repo_path.join(".git/config"),
            "[core]\n\trepositoryformatversion = 0\n\tbare = false\n",
        )
        .unwrap();

        assert!(!GitClient::has_resolvable_head(repo_path));
    }

    #[test]
    fn test_resolve_source_uses_subpath_tree_hash() {
        let temp_dir = init_test_repo();
        let source = SkillSource::GitHub {
            repo: "owner/repo".to_string(),
            path: Some("nested".to_string()),
            branch: Some("main".to_string()),
            tag: None,
            commit: None,
        };

        let resolved = GitClient::resolve_source(&source, temp_dir.path()).unwrap();

        assert_eq!(resolved.tree_hash.len(), 40);
        assert_eq!(resolved.reference, "branch:main");
    }
}
