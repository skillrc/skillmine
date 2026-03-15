    use super::*;
    use crate::cli::commands::{clean, doctor, info, outdated};
    use crate::cli::diagnostics::{DiagnosticLevel, DiagnosticSummary};
    use crate::cli::state::{classify_outdated, OutdatedState, SkillStatus};
    use crate::config::{Config, ConfigSkill, SkillSource};
    use crate::lockfile::LOCKFILE_NAME;
    use git2::{Repository, Signature};
    use tempfile::TempDir;
    use tokio::sync::{Mutex, OnceCell};

    fn commit_file(repo_path: &Path, relative_path: &str, content: &str, message: &str) {
        std::fs::write(repo_path.join(relative_path), content).unwrap();

        let repo = Repository::open(repo_path).unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(Path::new(relative_path)).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = Signature::now("skillmine", "skillmine@example.com").unwrap();

        let parent = repo
            .head()
            .ok()
            .and_then(|head| head.target())
            .and_then(|oid| repo.find_commit(oid).ok());

        match parent {
            Some(parent) => {
                repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    message,
                    &tree,
                    &[&parent],
                )
                .unwrap();
            }
            None => {
                repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])
                    .unwrap();
            }
        }
    }

    fn init_local_git_repo() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        Repository::init(temp_dir.path()).unwrap();
        commit_file(temp_dir.path(), "skill.md", "v1", "initial");
        temp_dir
    }

    async fn cwd_lock() -> &'static Mutex<()> {
        static LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();
        LOCK.get_or_init(|| async { Mutex::new(()) }).await
    }

    #[tokio::test]
    async fn test_init_local_creates_config_file() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = init(true).await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(temp_dir.path().join("skills.toml").exists());
    }

    #[tokio::test]
    async fn test_add_updates_local_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let initial = Config::default();
        crate::config::io::save_config(&initial, &temp_dir.path().join("skills.toml")).unwrap();

        let result = add("anthropic/skills/git-release".to_string(), None, None).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(updated.skills.contains_key("git-release"));
    }

    #[tokio::test]
    async fn test_add_rejects_invalid_repo_format() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let initial = Config::default();
        crate::config::io::save_config(&initial, &temp_dir.path().join("skills.toml")).unwrap();

        let result = add("/repo".to_string(), None, None).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(updated.skills.is_empty());
    }

    #[tokio::test]
    async fn test_install_with_empty_config_is_noop() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        crate::config::io::save_config(&Config::default(), &temp_dir.path().join("skills.toml"))
            .unwrap();

        let result = install(false, false).await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sync_local_skill_to_custom_target() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_dir = temp_dir.path().join("local-skill");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("README.md"), "skill").unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-skill",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir.to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let target_dir = temp_dir.path().join("target-skills");
        let result = sync(
            "claude".to_string(),
            Some(target_dir.to_string_lossy().to_string()),
        )
        .await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(target_dir.join("local-skill").exists());
    }

    #[tokio::test]
    async fn test_list_and_remove_flow() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        assert!(list(false).await.is_ok());
        assert!(remove("demo".to_string()).await.is_ok());

        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();
        std::env::set_current_dir(original_dir).unwrap();

        assert!(updated.skills.is_empty());
    }

    #[tokio::test]
    async fn test_freeze_and_thaw_flow() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        assert!(freeze().await.is_ok());
        assert!(temp_dir.path().join(LOCKFILE_NAME).exists());
        assert!(thaw().await.is_ok());

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_clean_removes_tmp_directory() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let tmp_dir = temp_dir.path().join("skillmine").join("tmp");
        std::fs::create_dir_all(&tmp_dir).unwrap();

        let previous = dirs::data_local_dir();
        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }

        let result = clean(false).await;

        if let Some(value) = previous {
            unsafe {
                std::env::set_var("XDG_DATA_HOME", value);
            }
        }

        assert!(result.is_ok());
        assert!(!tmp_dir.exists());
    }

    #[tokio::test]
    async fn test_update_promotes_locked_commit_into_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: "abc123resolved".to_string(),
                resolved_tree_hash: "tree123".to_string(),
                resolved_reference: "branch:main".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        let result = update(Some("demo".to_string())).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        match &updated.skills["demo"].source {
            SkillSource::GitHub { branch, tag, commit, .. } => {
                assert!(branch.is_none());
                assert!(tag.is_none());
                assert_eq!(commit.as_deref(), Some("abc123resolved"));
            }
            _ => panic!("expected github source"),
        }
    }

    #[tokio::test]
    async fn test_outdated_detects_missing_lock_entry() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = outdated().await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_install_refreshes_lockfile_for_local_skill() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_dir = temp_dir.path().join("local-skill");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("README.md"), "skill").unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-skill",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir.to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = install(false, false).await;
        let lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(lockfile.get_skill("local-skill").is_some());
    }

    #[tokio::test]
    async fn test_doctor_runs_with_lock_mismatch() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "configured-only",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "locked-only".to_string(),
                source_type: "version".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: Some("^2.0".to_string()),
                resolved_commit: "^2.0".to_string(),
                resolved_tree_hash: "^2.0".to_string(),
                resolved_reference: "version".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        let result = doctor().await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_outdated_detects_local_git_drift() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let initial_resolved = crate::registry::GitClient::resolve_local_head(local_repo.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "local-git".to_string(),
                source_type: "local".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: Some(local_repo.path().to_string_lossy().to_string()),
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(local_repo.path(), "skill.md", "v2", "second");

        let result = outdated().await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_refreshes_local_git_lock_entry() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let initial_resolved = crate::registry::GitClient::resolve_local_head(local_repo.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let mut lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "local-git".to_string(),
                source_type: "local".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: Some(local_repo.path().to_string_lossy().to_string()),
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(local_repo.path(), "skill.md", "v2", "second");
        let latest_resolved = crate::registry::GitClient::resolve_local_head(local_repo.path()).unwrap();

        let result = update(Some("local-git".to_string())).await;
        lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let entry = lockfile.get_skill("local-git").unwrap();
        assert_eq!(entry.resolved_commit, latest_resolved.commit);
        assert_eq!(entry.resolved_tree_hash, latest_resolved.tree_hash);
    }

    #[tokio::test]
    async fn test_outdated_detects_github_tmp_clone_drift() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let repo_dir = tmp_root_dir.join("demo");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&repo_dir).unwrap();
        commit_file(&repo_dir, "skill.md", "v1", "initial");
        let initial_resolved = crate::registry::GitClient::resolve_local_head(&repo_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(&repo_dir, "skill.md", "v2", "second");

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = outdated().await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_refreshes_github_tmp_clone_lock_entry() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let repo_dir = tmp_root_dir.join("demo");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&repo_dir).unwrap();
        commit_file(&repo_dir, "skill.md", "v1", "initial");
        let initial_resolved = crate::registry::GitClient::resolve_local_head(&repo_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let mut lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(&repo_dir, "skill.md", "v2", "second");
        let latest_resolved = crate::registry::GitClient::resolve_local_head(&repo_dir).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = update(Some("demo".to_string())).await;
        lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
        let entry = lockfile.get_skill("demo").unwrap();
        assert_eq!(entry.resolved_commit, latest_resolved.commit);
        assert_eq!(entry.resolved_tree_hash, latest_resolved.tree_hash);
    }

    #[tokio::test]
    async fn test_install_prefers_locked_tree_hash_for_github_tmp_clone() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let store_root_dir = temp_dir.path().join("skillmine").join("store");
        let repo_dir = tmp_root_dir.join("demo");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&repo_dir).unwrap();
        commit_file(&repo_dir, "skill.md", "v1", "initial");
        let initial_resolved = crate::registry::GitClient::resolve_local_head(&repo_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(&repo_dir, "skill.md", "v2", "second");

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = install(false, false).await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let refreshed_lock = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();
        let entry = refreshed_lock.get_skill("demo").unwrap();
        assert_eq!(entry.resolved_commit, initial_resolved.commit);
        assert!(store_root_dir.exists());
    }

    #[tokio::test]
    async fn test_sync_uses_locked_cached_content_for_github_skill() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let store = crate::installer::ContentStore::new(temp_dir.path().join("skillmine").join("store"));
        store.init().unwrap();
        let source_dir = temp_dir.path().join("cached-source");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("skill.md"), "cached").unwrap();
        let tree_hash = "ab1234567890";
        let stored_path = store.store(tree_hash, &source_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: "commit123".to_string(),
                resolved_tree_hash: tree_hash.to_string(),
                resolved_reference: "branch:main".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        let target_dir = temp_dir.path().join("target-skills");
        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = sync(
            "claude".to_string(),
            Some(target_dir.to_string_lossy().to_string()),
        )
        .await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let synced = target_dir.join("demo");
        assert!(synced.exists());
        let canonical = std::fs::canonicalize(&synced).unwrap();
        assert_eq!(canonical, stored_path);
    }

    #[tokio::test]
    async fn test_doctor_fails_for_invalid_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        std::fs::write(
            temp_dir.path().join("skills.toml"),
            r#"version = "2.0"

[settings]
concurrency = 0
timeout = 300
auto_sync = false
"#,
        )
        .unwrap();

        let result = doctor().await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_clean_all_removes_store_and_tmp() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let data_root = temp_dir.path().join("skillmine");
        let tmp_dir = data_root.join("tmp");
        let store_dir = data_root.join("store");
        std::fs::create_dir_all(&tmp_dir).unwrap();
        std::fs::create_dir_all(&store_dir).unwrap();
        std::fs::write(tmp_dir.join("temp.txt"), "tmp").unwrap();
        std::fs::write(store_dir.join("cache.txt"), "cache").unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = clean(true).await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        assert!(result.is_ok());
        assert!(!tmp_dir.exists());
        assert!(!store_dir.exists());
    }

    #[tokio::test]
    async fn test_doctor_warns_when_locked_cache_missing_after_clean_all() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "version".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: Some("^1.0".to_string()),
                resolved_commit: "^1.0".to_string(),
                resolved_tree_hash: "missing-cache".to_string(),
                resolved_reference: "version".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        let result = doctor().await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_doctor_reports_per_skill_status_lines() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = doctor().await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_outdated_marks_cache_missing_version_lock() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let state = classify_outdated(
            config.skills.get("demo").unwrap(),
            Some(&LockedSkill {
                name: "demo".to_string(),
                source_type: "version".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: Some("^1.0".to_string()),
                resolved_commit: "^1.0".to_string(),
                resolved_tree_hash: "missing-cache".to_string(),
                resolved_reference: "version".to_string(),
                resolved_at: Utc::now(),
            }),
        );

        std::env::set_current_dir(original_dir).unwrap();
        assert_eq!(state, OutdatedState::CacheMissing);
    }

    #[tokio::test]
    async fn test_classify_outdated_marks_pinned_github_skill() {
        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: None,
                tag: None,
                commit: Some("abc123".to_string()),
            },
            name: None,
        };

        let state = classify_outdated(
            &skill,
            Some(&LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: Some("abc123".to_string()),
                local_path: None,
                version_constraint: None,
                resolved_commit: "abc123".to_string(),
                resolved_tree_hash: "tree123".to_string(),
                resolved_reference: "commit".to_string(),
                resolved_at: Utc::now(),
            }),
        );

        assert_eq!(state, OutdatedState::Pinned);
    }

    #[tokio::test]
    async fn test_classify_outdated_marks_tmp_missing_for_github_without_clone() {
        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            },
            name: None,
        };

        let state = classify_outdated(
            &skill,
            Some(&LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: "abc123".to_string(),
                resolved_tree_hash: "tree123".to_string(),
                resolved_reference: "branch:main".to_string(),
                resolved_at: Utc::now(),
            }),
        );

        assert_eq!(state, OutdatedState::TmpMissing);
    }

    #[test]
    fn test_describe_skill_source_for_github_commit() {
        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: Some("sub/skill".to_string()),
                branch: None,
                tag: None,
                commit: Some("1234567890abcdef".to_string()),
            },
            name: None,
        };

        let rendered = describe_skill_source(&skill);
        assert!(rendered.contains("github:owner/repo"));
        assert!(rendered.contains("path=sub/skill"));
        assert!(rendered.contains("commit=12345678"));
    }

    #[test]
    fn test_describe_locked_skill_formats_short_hashes() {
        let locked = LockedSkill {
            name: "demo".to_string(),
            source_type: "github".to_string(),
            repo: Some("owner/repo".to_string()),
            path: None,
            requested_branch: Some("main".to_string()),
            requested_tag: None,
            requested_commit: None,
            local_path: None,
            version_constraint: None,
            resolved_commit: "abcdef1234567890".to_string(),
            resolved_tree_hash: "fedcba0987654321".to_string(),
            resolved_reference: "branch:main".to_string(),
            resolved_at: Utc::now(),
        };

        let rendered = describe_locked_skill(Some(&locked));
        assert!(rendered.contains("locked=abcdef12"));
        assert!(rendered.contains("tree=fedcba09"));
        assert!(rendered.contains("ref=branch:main"));
    }

    #[test]
    fn test_describe_locked_skill_none() {
        assert_eq!(describe_locked_skill(None), "locked=none");
    }

    #[test]
    fn test_cli_pure_module_exports_description_helpers() {
        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: Some("nested/skill".to_string()),
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            },
            name: Some("skill".to_string()),
        };

        let locked = LockedSkill {
            name: "skill".to_string(),
            source_type: "github".to_string(),
            repo: Some("owner/repo".to_string()),
            path: Some("nested/skill".to_string()),
            requested_branch: Some("main".to_string()),
            requested_tag: None,
            requested_commit: None,
            local_path: None,
            version_constraint: None,
            resolved_commit: "1234567890abcdef1234567890abcdef12345678".to_string(),
            resolved_tree_hash: "abcdef1234567890abcdef1234567890abcdef12".to_string(),
            resolved_reference: "refs/heads/main".to_string(),
            resolved_at: Utc::now(),
        };

        let source = crate::cli::pure::describe_skill_source(&skill);
        let summary = crate::cli::pure::describe_locked_skill(Some(&locked));

        assert!(source.contains("github:owner/repo"));
        assert!(source.contains("path=nested/skill"));
        assert!(summary.contains("locked=12345678"));
    }

    #[test]
    fn test_diagnostic_summary_counts_levels() {
        let mut summary = DiagnosticSummary::default();
        summary.record(DiagnosticLevel::Pass);
        summary.record(DiagnosticLevel::Warn);
        summary.record(DiagnosticLevel::Fail);
        summary.record(DiagnosticLevel::Warn);

        assert_eq!(summary.pass, 1);
        assert_eq!(summary.warn, 2);
        assert_eq!(summary.fail, 1);
    }

    #[tokio::test]
    async fn test_mixed_multi_skill_workflow_regression() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let github_tmp = tmp_root_dir.join("github-skill");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&github_tmp).unwrap();
        commit_file(&github_tmp, "skill.md", "gh-v1", "initial");

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.add_skill(
            "github-skill",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.add_skill(
            "version-skill",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        assert!(install(false, false).await.is_ok());
        assert!(freeze().await.is_ok());
        assert!(list(true).await.is_ok());
        assert!(outdated().await.is_ok());
        assert!(doctor().await.is_ok());
        assert!(clean(false).await.is_ok());
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();
        assert!(temp_dir.path().join(LOCKFILE_NAME).exists());
    }

    #[tokio::test]
    async fn test_outdated_does_not_mark_tmp_missing_when_valid_github_tmp_exists() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let github_tmp = tmp_root_dir.join("github-skill");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&github_tmp).unwrap();
        commit_file(&github_tmp, "skill.md", "gh-v1", "initial");
        let resolved = crate::registry::GitClient::resolve_local_head(&github_tmp).unwrap();

        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            },
            name: None,
        };

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }

        let state = classify_outdated(
            &skill,
            Some(&LockedSkill {
                name: "github-skill".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: resolved.commit,
                resolved_tree_hash: resolved.tree_hash,
                resolved_reference: resolved.reference,
                resolved_at: Utc::now(),
            }),
        );

        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();
        assert_eq!(state, OutdatedState::UpToDate);
    }

    #[tokio::test]
    async fn test_end_to_end_local_lock_workflow() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let target_dir = temp_dir.path().join("skills-target");

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        assert!(install(false, false).await.is_ok());
        assert!(freeze().await.is_ok());
        commit_file(local_repo.path(), "skill.md", "v2", "second");
        assert!(outdated().await.is_ok());
        assert!(update(Some("local-git".to_string())).await.is_ok());
        assert!(thaw().await.is_ok());
        assert!(sync("claude".to_string(), Some(target_dir.to_string_lossy().to_string())).await.is_ok());
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(temp_dir.path().join(LOCKFILE_NAME).exists());
        assert!(target_dir.join("local-git").exists());
    }

    #[tokio::test]
    async fn test_remove_cleans_lock_and_tmp_clone() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let skill_tmp = tmp_root_dir.join("demo");
        std::fs::create_dir_all(&skill_tmp).unwrap();
        std::fs::write(skill_tmp.join("skill.md"), "temp").unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "version".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: Some("^1.0".to_string()),
                resolved_commit: "^1.0".to_string(),
                resolved_tree_hash: "^1.0".to_string(),
                resolved_reference: "version".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = remove("demo".to_string()).await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();
        let updated_lock = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(updated.skills.is_empty());
        assert!(updated_lock.skills.is_empty());
        assert!(!skill_tmp.exists());
    }

    #[tokio::test]
    async fn test_list_should_not_mark_uninstalled_skill_as_installed_from_global_tmp() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "git-release",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                    branch: None,
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let statuses = skill_statuses(
            "git-release",
            config.skills.get("git-release").unwrap(),
            None,
            &temp_dir.path().join("skillmine").join("tmp"),
            &crate::installer::ContentStore::new(temp_dir.path().join("skillmine").join("store")),
        );

        std::env::set_current_dir(original_dir).unwrap();

        assert_eq!(statuses, vec![SkillStatus::Configured]);
    }

    #[tokio::test]
    async fn test_freeze_fails_cleanly_for_uninstalled_github_skill() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "git-release",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                    branch: None,
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = freeze().await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        let message = result.err().unwrap().to_string();
        assert!(message.contains("run install first") || message.contains("unresolved"));
    }

    #[tokio::test]
    async fn test_freeze_recovers_after_broken_tmp_git_repo_is_replaced() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let broken_tmp = temp_dir.path().join("skillmine").join("tmp").join("git-release");
        std::fs::create_dir_all(broken_tmp.join(".git/refs/heads")).unwrap();
        std::fs::write(broken_tmp.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        std::fs::write(
            broken_tmp.join(".git/config"),
            "[core]\n\trepositoryformatversion = 0\n\tbare = false\n",
        )
        .unwrap();

        let mut config = Config::default();
        config.add_skill(
            "git-release",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                    branch: None,
                    tag: None,
                    commit: Some("1234567890abcdef1234567890abcdef12345678".to_string()),
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = freeze().await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(broken_tmp.exists());
    }

    #[tokio::test]
    async fn test_install_removes_broken_tmp_repo_before_reusing_it() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let broken_tmp = temp_dir.path().join("skillmine").join("tmp").join("git-release");
        std::fs::create_dir_all(broken_tmp.join(".git/refs/heads")).unwrap();
        std::fs::write(broken_tmp.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        std::fs::write(
            broken_tmp.join(".git/config"),
            "[core]\n\trepositoryformatversion = 0\n\tbare = false\n",
        )
        .unwrap();

        let mut config = Config::default();
        config.add_skill(
            "git-release",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                    branch: None,
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = install(false, false).await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err() || result.is_ok());
        assert!(!broken_tmp.exists());
    }

    #[tokio::test]
    async fn test_install_concurrent_local_skills_refreshes_lockfile_for_all() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_dir_a = temp_dir.path().join("local-skill-a");
        let source_dir_b = temp_dir.path().join("local-skill-b");
        std::fs::create_dir_all(&source_dir_a).unwrap();
        std::fs::create_dir_all(&source_dir_b).unwrap();
        std::fs::write(source_dir_a.join("README.md"), "skill-a").unwrap();
        std::fs::write(source_dir_b.join("README.md"), "skill-b").unwrap();

        let mut config = Config::default();
        config.settings.concurrency = 2;
        config.add_skill(
            "local-skill-a",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir_a.to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.add_skill(
            "local-skill-b",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir_b.to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = install(false, false).await;
        let lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(lockfile.get_skill("local-skill-a").is_some());
        assert!(lockfile.get_skill("local-skill-b").is_some());
    }

    #[tokio::test]
    async fn test_install_concurrent_mixed_skills_keeps_successful_local_results() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_dir = temp_dir.path().join("local-skill");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("README.md"), "skill").unwrap();

        let mut config = Config::default();
        config.settings.concurrency = 3;
        config.add_skill(
            "local-skill",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir.to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.add_skill(
            "missing-local",
            ConfigSkill {
                source: SkillSource::Local {
                    path: temp_dir.path().join("missing").to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.add_skill(
            "version-skill",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = install(false, false).await;
        let lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(lockfile.get_skill("local-skill").is_some());
        assert!(lockfile.get_skill("missing-local").is_none());
        assert!(lockfile.get_skill("version-skill").is_some());
    }

    #[tokio::test]
    async fn test_install_resolves_version_skill_through_registry() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_repo = init_local_git_repo();
        let repo = Repository::open(source_repo.path()).unwrap();
        let head = repo.head().unwrap().peel_to_commit().unwrap();
        repo.tag_lightweight("v1.2.3", head.as_object(), false).unwrap();
        let resolved = crate::registry::GitClient::resolve_local_head(source_repo.path()).unwrap();

        let mut config = Config::default();
        config.registry.insert(
            "python-testing".to_string(),
            crate::config::RegistryEntry {
                repo: source_repo.path().to_string_lossy().to_string(),
                path: None,
            },
        );
        config.add_skill(
            "python-testing",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        unsafe { std::env::set_var("XDG_DATA_HOME", temp_dir.path()); }
        let result = install(false, false).await;
        let lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();
        unsafe { std::env::remove_var("XDG_DATA_HOME"); }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let entry = lockfile.get_skill("python-testing").unwrap();
        assert_eq!(entry.source_type, "github");
        assert_eq!(entry.repo.as_deref(), Some(source_repo.path().to_string_lossy().as_ref()));
        assert_eq!(entry.path, None);
        assert_eq!(entry.version_constraint.as_deref(), Some("^1.0"));
        assert_eq!(entry.requested_tag.as_deref(), Some("v1.2.3"));
        assert_eq!(entry.resolved_commit, resolved.commit);
    }

    #[tokio::test]
    async fn test_install_version_skill_fails_without_registry_entry() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "python-testing",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = install(false, false).await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();
        let entry = lockfile.get_skill("python-testing").unwrap();
        assert_eq!(entry.source_type, "version");
        assert_eq!(entry.version_constraint.as_deref(), Some("^1.0"));
    }

    #[tokio::test]
    async fn test_info_reports_skill_details() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: Some("subdir".to_string()),
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = info("demo".to_string()).await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }
