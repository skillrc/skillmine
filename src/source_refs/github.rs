// LEGACY CODE
// This module contains GitHub remote operation logic.
// Remote installation from GitHub is currently DISABLED in the main flow.
// See: src/cli/mod.rs guard_add_skill_input() which explicitly rejects remote paths.
// This code is retained for possible future extensions but should NOT be used
// in core local-first workflows. Refer to ARCHITECTURE.md "Local-first direction".

use crate::config::SkillSource;
use crate::error::Result;
use crate::error::SkillmineError;
use crate::source_refs::pure::{github_repo_url, reference_for_requested_ref};
use git2::{build::RepoBuilder, FetchOptions, Oid, Repository};
use std::path::Path;

pub struct GitClient;

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
                "Version-based resolution is not supported in the local-first flow".to_string(),
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
                "Version-based resolution is not supported in the local-first flow".to_string(),
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
