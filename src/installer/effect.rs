use crate::error::{Result, SkillmineError};
use std::fs;
use std::path::{Path, PathBuf};

pub fn content_path_for_root(root: &Path, tree_hash: &str) -> PathBuf {
    let prefix = &tree_hash[..2.min(tree_hash.len())];
    let rest = &tree_hash[2.min(tree_hash.len())..];
    root.join(prefix).join(rest)
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).map_err(SkillmineError::Io)?;

    for entry in fs::read_dir(src).map_err(SkillmineError::Io)? {
        let entry = entry.map_err(SkillmineError::Io)?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(SkillmineError::Io)?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn hard_link_or_copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).map_err(SkillmineError::Io)?;

    for entry in fs::read_dir(src).map_err(SkillmineError::Io)? {
        let entry = entry.map_err(SkillmineError::Io)?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            hard_link_or_copy_dir(&src_path, &dst_path)?;
        } else {
            match fs::hard_link(&src_path, &dst_path) {
                Ok(_) => {}
                Err(_) => {
                    fs::copy(&src_path, &dst_path).map_err(SkillmineError::Io)?;
                }
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn hard_link_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).map_err(SkillmineError::Io)?;

    for entry in fs::read_dir(src).map_err(SkillmineError::Io)? {
        let entry = entry.map_err(SkillmineError::Io)?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            hard_link_dir(&src_path, &dst_path)?;
        } else {
            fs::hard_link(&src_path, &dst_path).map_err(SkillmineError::Io)?;
        }
    }

    Ok(())
}
