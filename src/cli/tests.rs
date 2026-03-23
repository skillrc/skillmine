    use super::*;
    use crate::cli::commands::{clean, doctor, doctor_summary, info, outdated};
    use crate::cli::diagnostics::{DiagnosticLevel, DiagnosticSummary};
    use crate::cli::state::{classify_outdated, OutdatedState, SkillStatus};
    use crate::config::{Config, ConfigSkill, SkillSource};
    use crate::resolved_state::LOCKFILE_NAME;
    use chrono::Utc;
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
    async fn test_create_generates_local_skill_and_guides_next_steps() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Create a local skills.toml with no workspace so create falls back to XDG_DATA_HOME
        crate::config::io::save_config(
            &Config::default(),
            &temp_dir.path().join("skills.toml"),
        )
        .unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }

        let result = crate::cli::create::create("demo-skill".to_string(), None).await;

        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let output = result.unwrap();
        let skill_dir = temp_dir.path().join("skillmine").join("skills").join("demo-skill");

        assert!(skill_dir.exists());
        assert!(skill_dir.join("SKILL.toml").exists());
        assert!(skill_dir.join("SKILL.md").exists());
        assert!(skill_dir.join("README.md").exists());

        let manifest = crate::manifest::load_manifest(&skill_dir, &None)
            .unwrap()
            .expect("manifest should load");
        assert_eq!(manifest.skill.name, "demo-skill");
        assert_eq!(manifest.skill.version, "0.1.0");

        assert!(output.contains("Created skill package at"));
        assert!(output.contains("skillmine add"));
        assert!(output.contains("skillmine sync --target=opencode"));
    }

    #[tokio::test]
    async fn test_create_uses_output_directory_when_provided() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let output_root = temp_dir.path().join("generated-skills");
        let result = crate::cli::create::create(
            "demo-skill".to_string(),
            Some(output_root.to_string_lossy().to_string()),
        )
        .await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let skill_dir = output_root.join("demo-skill");
        assert!(skill_dir.join("SKILL.toml").exists());
        assert!(result.unwrap().contains(&skill_dir.to_string_lossy().to_string()));
    }

    #[tokio::test]
    async fn test_create_uses_workspace_from_config_when_present() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let workspace_dir = temp_dir.path().join("workspace-skills");
        let mut config = Config::default();
        config.settings.workspace = Some(workspace_dir.to_string_lossy().to_string());
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = crate::cli::create::create("demo-skill".to_string(), None).await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let skill_dir = workspace_dir.join("demo-skill");
        assert!(skill_dir.join("SKILL.toml").exists());
        assert!(result.unwrap().contains(&skill_dir.to_string_lossy().to_string()));
    }

    #[tokio::test]
    async fn test_create_and_add_generates_skill_and_registers_local_source() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }

        crate::config::io::save_config(&Config::default(), &temp_dir.path().join("skills.toml"))
            .unwrap();

        let result = create_asset_and_add("demo-skill".to_string(), None, "skill").await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let output = result.unwrap();
        let skill_dir = temp_dir.path().join("skillmine").join("skills").join("demo-skill");
        assert!(skill_dir.join("SKILL.toml").exists());
        let skill = updated.skills.get("demo-skill").unwrap();
        match &skill.source {
            SkillSource::Local { path } => {
                assert_eq!(path, &skill_dir.to_string_lossy().to_string());
            }
            other => panic!("expected local source, got {other:?}"),
        }
        assert!(output.contains("Created skill package at"));
        assert!(output.contains("Added local source '"));
    }

    #[tokio::test]
    async fn test_config_set_persists_workspace_value() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", temp_dir.path());
        }

        let result = config_set("workspace".to_string(), "~/Project/Skills".to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skillmine").join("skills.toml")).unwrap();

        unsafe {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert_eq!(updated.settings.workspace.as_deref(), Some("~/Project/Skills"));
    }

    #[tokio::test]
    async fn test_config_show_reports_workspace_value() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", temp_dir.path());
        }

        let mut config = Config::default();
        config.settings.workspace = Some("~/Project/Skills".to_string());
        let config_path = temp_dir.path().join("skillmine").join("skills.toml");
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        crate::config::io::save_config(&config, &config_path).unwrap();

        let result = config_show().await;

        unsafe {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("workspace = ~/Project/Skills"));
        assert!(output.contains("skills.toml"));
    }

    #[tokio::test]
    async fn test_create_and_add_without_config_fails_before_scaffold() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = create_asset_and_add("demo-skill".to_string(), None, "skill").await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(!temp_dir.path().join("demo-skill").exists());
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
    async fn test_add_rejects_remote_path_and_keeps_config_empty() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let initial = Config::default();
        crate::config::io::save_config(&initial, &temp_dir.path().join("skills.toml")).unwrap();

        let result = add("anthropic/skills/git-release".to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(updated.skills.is_empty());
    }

    #[tokio::test]
    async fn test_add_local_path_updates_local_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let initial = Config::default();
        crate::config::io::save_config(&initial, &temp_dir.path().join("skills.toml")).unwrap();

        let local_skill_dir = temp_dir.path().join("opencode-skill-local-demo");
        std::fs::create_dir_all(&local_skill_dir).unwrap();
        std::fs::write(local_skill_dir.join("README.md"), "demo").unwrap();

        let result = add(local_skill_dir.to_string_lossy().to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let skill = updated.skills.get("opencode-skill-local-demo").unwrap();
        match &skill.source {
            SkillSource::Local { path } => {
                assert_eq!(path, &local_skill_dir.to_string_lossy().to_string());
            }
            other => panic!("expected local source, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_add_detects_agent_type() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        crate::config::io::save_config(&Config::default(), &temp_dir.path().join("skills.toml"))
            .unwrap();

        let agent_dir = temp_dir.path().join("planner-agent");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(agent_dir.join("AGENT.md"), "---\ndescription: demo\n---\nbody\n").unwrap();

        let result = add(agent_dir.to_string_lossy().to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(updated.skills.is_empty());
        assert!(updated.commands.is_empty());
        let agent = updated.agents.get("planner-agent").unwrap();
        match &agent.source {
            SkillSource::Local { path } => {
                assert_eq!(path, &agent_dir.to_string_lossy().to_string());
            }
            other => panic!("expected local source, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_add_detects_command_type() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        crate::config::io::save_config(&Config::default(), &temp_dir.path().join("skills.toml"))
            .unwrap();

        let command_dir = temp_dir.path().join("pre-commit");
        std::fs::create_dir_all(&command_dir).unwrap();
        std::fs::write(command_dir.join("COMMAND.md"), "---\ndescription: demo\n---\nbody\n").unwrap();

        let result = add(command_dir.to_string_lossy().to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(updated.skills.is_empty());
        assert!(updated.agents.is_empty());
        let command = updated.commands.get("pre-commit").unwrap();
        match &command.source {
            SkillSource::Local { path } => {
                assert_eq!(path, &command_dir.to_string_lossy().to_string());
            }
            other => panic!("expected local source, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_add_rejects_invalid_repo_format() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let initial = Config::default();
        crate::config::io::save_config(&initial, &temp_dir.path().join("skills.toml")).unwrap();

        let result = add("/repo".to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(updated.skills.is_empty());
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
            ConfigSkill { source: SkillSource::Local {
                path: source_dir.to_string_lossy().to_string(),
            }, name: None, enabled: true, sync_enabled: true },
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
        assert!(result.unwrap().contains("✓ Synced 'local-skill' ->"));
    }

    #[tokio::test]
    async fn test_sync_routes_agent_to_agents_dir() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_dir = temp_dir.path().join("planner-agent");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("AGENT.md"), "---\ndescription: demo\n---\nbody\n").unwrap();

        let mut config = Config::default();
        config.add_agent(
            "planner-agent",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir.to_string_lossy().to_string(),
                },
                name: None,
                enabled: true,
                sync_enabled: true,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", temp_dir.path());
        }
        let result = sync("opencode".to_string(), None).await;
        unsafe {
            std::env::remove_var("XDG_CONFIG_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let target = temp_dir.path().join("opencode").join("agents").join("planner-agent.md");
        assert!(target.exists());
        assert!(target.is_symlink());
        assert!(result.unwrap().contains("✓ Synced agent 'planner-agent' ->"));
    }

    #[tokio::test]
    async fn test_sync_routes_command_to_commands_dir() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_dir = temp_dir.path().join("pre-commit");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("COMMAND.md"), "---\ndescription: demo\n---\nbody\n").unwrap();

        let mut config = Config::default();
        config.add_command(
            "pre-commit",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir.to_string_lossy().to_string(),
                },
                name: None,
                enabled: true,
                sync_enabled: true,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", temp_dir.path());
        }
        let result = sync("opencode".to_string(), None).await;
        unsafe {
            std::env::remove_var("XDG_CONFIG_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let target = temp_dir.path().join("opencode").join("commands").join("pre-commit.md");
        assert!(target.exists());
        assert!(target.is_symlink());
        assert!(result.unwrap().contains("✓ Synced command 'pre-commit' ->"));
    }

    #[test]
    fn test_load_skill_summaries_includes_agents_and_commands() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let agent_dir = temp_dir.path().join("planner-agent");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(agent_dir.join("AGENT.md"), "---\ndescription: agent\n---\nbody\n").unwrap();

        let command_dir = temp_dir.path().join("pre-commit");
        std::fs::create_dir_all(&command_dir).unwrap();
        std::fs::write(command_dir.join("COMMAND.md"), "---\ndescription: command\n---\nbody\n").unwrap();

        let mut config = Config::default();
        config.add_agent(
            "planner-agent",
            ConfigSkill {
                source: SkillSource::Local {
                    path: agent_dir.to_string_lossy().to_string(),
                },
                name: None,
                enabled: true,
                sync_enabled: true,
            },
        );
        config.add_command(
            "pre-commit",
            ConfigSkill {
                source: SkillSource::Local {
                    path: command_dir.to_string_lossy().to_string(),
                },
                name: None,
                enabled: true,
                sync_enabled: true,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let summaries = load_skill_summaries().unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(summaries.iter().any(|s| s.name == "planner-agent" && s.asset_type == "agent"));
        assert!(summaries.iter().any(|s| s.name == "pre-commit" && s.asset_type == "command"));
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
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        assert!(list(false).await.is_ok());
        assert!(remove("demo".to_string(), false).await.is_ok());

        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();
        std::env::set_current_dir(original_dir).unwrap();

        assert!(updated.skills.is_empty());
    }

    #[tokio::test]
    async fn test_disable_marks_skill_as_disabled_in_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = disable("demo".to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(!updated.skills.get("demo").unwrap().enabled);
    }

    #[tokio::test]
    async fn test_enable_marks_skill_as_enabled_in_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: false, sync_enabled: true },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = enable("demo".to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(updated.skills.get("demo").unwrap().enabled);
    }

    #[tokio::test]
    async fn test_unsync_marks_skill_as_runtime_unsynced_in_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = unsync("demo".to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(!updated.skills.get("demo").unwrap().sync_enabled);
    }

    #[tokio::test]
    async fn test_resync_marks_skill_as_runtime_synced_in_config() {
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
                enabled: true,
                sync_enabled: false,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = resync("demo".to_string()).await;
        let updated = crate::config::io::load_config(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(updated.skills.get("demo").unwrap().sync_enabled);
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
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
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
            ConfigSkill { source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            }, name: None, enabled: true, sync_enabled: true },
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
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = outdated().await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
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
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
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
        let initial_resolved = crate::source_refs::GitClient::resolve_local_head(local_repo.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill { source: SkillSource::Local {
                path: local_repo.path().to_string_lossy().to_string(),
            }, name: None, enabled: true, sync_enabled: true },
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
        let initial_resolved = crate::source_refs::GitClient::resolve_local_head(local_repo.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill { source: SkillSource::Local {
                path: local_repo.path().to_string_lossy().to_string(),
            }, name: None, enabled: true, sync_enabled: true },
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
        let latest_resolved = crate::source_refs::GitClient::resolve_local_head(local_repo.path()).unwrap();

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
        let initial_resolved = crate::source_refs::GitClient::resolve_local_head(&repo_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill { source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            }, name: None, enabled: true, sync_enabled: true },
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
        let initial_resolved = crate::source_refs::GitClient::resolve_local_head(&repo_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill { source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            }, name: None, enabled: true, sync_enabled: true },
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
        let latest_resolved = crate::source_refs::GitClient::resolve_local_head(&repo_dir).unwrap();

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
    async fn test_doctor_fails_for_invalid_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        std::fs::write(
            temp_dir.path().join("skills.toml"),
            r#"version = "2.0"

[settings]
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
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
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
            ConfigSkill { source: SkillSource::Local {
                path: local_repo.path().to_string_lossy().to_string(),
            }, name: None, enabled: true, sync_enabled: true },
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
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
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
        let skill = ConfigSkill { source: SkillSource::GitHub {
            repo: "owner/repo".to_string(),
            path: None,
            branch: None,
            tag: None,
            commit: Some("abc123".to_string()),
        }, name: None, enabled: true, sync_enabled: true };

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
        let skill = ConfigSkill { source: SkillSource::GitHub {
            repo: "owner/repo".to_string(),
            path: None,
            branch: Some("main".to_string()),
            tag: None,
            commit: None,
        }, name: None, enabled: true, sync_enabled: true };

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
        let skill = ConfigSkill { source: SkillSource::GitHub {
            repo: "owner/repo".to_string(),
            path: Some("sub/skill".to_string()),
            branch: None,
            tag: None,
            commit: Some("1234567890abcdef".to_string()),
        }, name: None, enabled: true, sync_enabled: true };

        let rendered = describe_skill_source(&skill);
        assert!(rendered.contains("GitHub: owner/repo"));
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
        let skill = ConfigSkill { source: SkillSource::GitHub {
            repo: "owner/repo".to_string(),
            path: Some("nested/skill".to_string()),
            branch: Some("main".to_string()),
            tag: None,
            commit: None,
        }, name: Some("skill".to_string()), enabled: true, sync_enabled: true };

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

        assert!(source.contains("GitHub: owner/repo"));
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
        let resolved = crate::source_refs::GitClient::resolve_local_head(&github_tmp).unwrap();

        let skill = ConfigSkill { source: SkillSource::GitHub {
            repo: "owner/repo".to_string(),
            path: None,
            branch: Some("main".to_string()),
            tag: None,
            commit: None,
        }, name: None, enabled: true, sync_enabled: true };

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
            ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: true, sync_enabled: true },
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
        let result = remove("demo".to_string(), false).await;
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
            ConfigSkill { source: SkillSource::GitHub {
                repo: "anthropic/skills".to_string(),
                path: Some("git-release".to_string()),
                branch: None,
                tag: None,
                commit: None,
            }, name: None, enabled: true, sync_enabled: true },
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
            ConfigSkill { source: SkillSource::GitHub {
                repo: "anthropic/skills".to_string(),
                path: Some("git-release".to_string()),
                branch: None,
                tag: None,
                commit: None,
            }, name: None, enabled: true, sync_enabled: true },
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
            ConfigSkill { source: SkillSource::GitHub {
                repo: "anthropic/skills".to_string(),
                path: Some("git-release".to_string()),
                branch: None,
                tag: None,
                commit: Some("1234567890abcdef1234567890abcdef12345678".to_string()),
            }, name: None, enabled: true, sync_enabled: true },
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
    async fn test_info_reports_skill_details() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill { source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: Some("subdir".to_string()),
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            }, name: None, enabled: true, sync_enabled: true },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let result = info("demo".to_string()).await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_disabled_skill_flag() {
        let toml_str = r#"
version = "1.0"

[skills]
demo = { repo = "owner/repo", enabled = false }
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        let skill = config.skills.get("demo").unwrap();

        assert!(!skill.enabled);
    }

    #[tokio::test]
    async fn test_sync_skips_disabled_skill() {
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
            ConfigSkill { source: SkillSource::Local {
                path: source_dir.to_string_lossy().to_string(),
            }, name: None, enabled: false, sync_enabled: true, },
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
        assert!(!target_dir.join("local-skill").exists());
        assert!(result.unwrap().contains("- Skipping 'local-skill' (disabled)"));
    }

    #[tokio::test]
    async fn test_sync_skips_unsynced_skill() {
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
                enabled: true,
                sync_enabled: false,
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
        assert!(!target_dir.join("local-skill").exists());
        assert!(result.unwrap().contains("- Skipping 'local-skill' (unsynced)"));
    }

    #[tokio::test]
    async fn test_tui_sync_api_returns_report_text() {
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
                enabled: true,
                sync_enabled: true,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let target_dir = temp_dir.path().join("tui-target-skills");
        let report = sync(
            "claude".to_string(),
            Some(target_dir.to_string_lossy().to_string()),
        )
        .await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(report.is_ok());
        assert!(report.unwrap().contains("✓ Synced 'local-skill' ->"));
    }

    #[test]
    fn test_disabled_skill_status_includes_disabled() {
        let skill = ConfigSkill { source: SkillSource::Version("^1.0".to_string()), name: None, enabled: false, sync_enabled: true, };

        let statuses = crate::cli::state::skill_statuses(
            "demo",
            &skill,
            None,
            Path::new("."),
            &crate::installer::ContentStore::default(),
        );

        assert_eq!(statuses, vec![SkillStatus::Configured, SkillStatus::Disabled]);
    }

    #[test]
    fn test_unsynced_skill_status_includes_unsynced() {
        let skill = ConfigSkill {
            source: SkillSource::Version("^1.0".to_string()),
            name: None,
            enabled: true,
            sync_enabled: false,
        };

        let statuses = crate::cli::state::skill_statuses(
            "demo",
            &skill,
            None,
            Path::new("."),
            &crate::installer::ContentStore::default(),
        );

        assert_eq!(statuses, vec![SkillStatus::Configured, SkillStatus::Unsynced]);
    }

    #[tokio::test]
    async fn test_doctor_summary_treats_disabled_skill_as_intentionally_inactive() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill { source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            }, name: None, enabled: false, sync_enabled: true, },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let summary = doctor_summary().await.unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(summary.contains("demo :: intentionally-inactive :: disabled"));
        assert!(summary.contains("Summary: 1 pass, 1 inactive, 0 warn, 0 fail"));
    }

    #[tokio::test]
    async fn test_doctor_summary_treats_unsynced_skill_as_runtime_inactive() {
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
                enabled: true,
                sync_enabled: false,
            },
        );
        crate::config::io::save_config(&config, &temp_dir.path().join("skills.toml")).unwrap();

        let summary = doctor_summary().await.unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(summary.contains("demo :: runtime-inactive :: unsynced"));
        assert!(summary.contains("Summary: 1 pass, 1 inactive, 0 warn, 0 fail"));
    }
