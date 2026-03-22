#![allow(dead_code)]
use crate::error::{Result, SkillmineError};
use std::fs;
use std::path::{Path, PathBuf};

// Platform-specific symlink imports
#[cfg(unix)]
use std::os::unix::fs::symlink as unix_symlink;
#[cfg(windows)]
use std::os::windows::fs::{symlink_dir, symlink_file};

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

/// Waterflow Architecture: Symlink sync effect
/// Creates symlinks from src to dst instead of copying
/// This allows source modifications to immediately reflect in runtime
#[cfg(unix)]
pub fn symlink_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).map_err(SkillmineError::Io)?;

    for entry in fs::read_dir(src).map_err(SkillmineError::Io)? {
        let entry = entry.map_err(SkillmineError::Io)?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            symlink_dir_all(&src_path, &dst_path)?;
        } else {
            // Remove existing symlink if broken
            if dst_path.is_symlink() && !dst_path.exists() {
                fs::remove_file(&dst_path).map_err(SkillmineError::Io)?;
            }
            unix_symlink(&src_path, &dst_path).map_err(SkillmineError::Io)?;
        }
    }

    Ok(())
}

#[cfg(windows)]
pub fn symlink_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).map_err(SkillmineError::Io)?;

    for entry in fs::read_dir(src).map_err(SkillmineError::Io)? {
        let entry = entry.map_err(SkillmineError::Io)?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            symlink_dir_all(&src_path, &dst_path)?;
        } else {
            // Remove existing symlink if broken
            if dst_path.is_symlink() && !dst_path.exists() {
                fs::remove_file(&dst_path).map_err(SkillmineError::Io)?;
            }
            symlink_file(&src_path, &dst_path).map_err(SkillmineError::Io)?;
        }
    }

    Ok(())
}

/// Clean up broken symlinks in a directory
pub fn clean_broken_symlinks(dir: &Path) -> Result<usize> {
    let mut removed = 0;

    if !dir.exists() {
        return Ok(0);
    }

    for entry in fs::read_dir(dir).map_err(SkillmineError::Io)? {
        let entry = entry.map_err(SkillmineError::Io)?;
        let path = entry.path();

        if path.is_symlink() && !path.exists() {
            fs::remove_file(&path).map_err(SkillmineError::Io)?;
            removed += 1;
        } else if path.is_dir() {
            removed += clean_broken_symlinks(&path)?;
        }
    }

    Ok(removed)
}
