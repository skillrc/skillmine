use crate::error::{Result, SkillmineError};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ContentStore {
    root: PathBuf,
}

impl ContentStore {
    pub fn default_path() -> Result<PathBuf> {
        dirs::data_dir()
            .map(|dir| dir.join("skillmine").join("store"))
            .ok_or_else(|| SkillmineError::Config("Failed to get data directory".to_string()))
    }

    pub fn new(path: PathBuf) -> Self {
        Self { root: path }
    }

    pub fn default() -> Self {
        Self::new(Self::default_path().unwrap_or_else(|_| PathBuf::from(".skillmine/store")))
    }

    pub fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.root).map_err(SkillmineError::Io)?;
        Ok(())
    }

    fn content_path(&self, tree_hash: &str) -> PathBuf {
        let prefix = &tree_hash[..2.min(tree_hash.len())];
        let rest = &tree_hash[2.min(tree_hash.len())..];
        self.root.join(prefix).join(rest)
    }

    #[cfg(test)]
    pub fn has_content(&self, tree_hash: &str) -> bool {
        self.content_path(tree_hash).exists()
    }

    pub fn store(&self, tree_hash: &str, source_path: &Path) -> Result<PathBuf> {
        let dest_path = self.content_path(tree_hash);

        if dest_path.exists() {
            return Ok(dest_path);
        }

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).map_err(SkillmineError::Io)?;
        }

        copy_dir_all(source_path, &dest_path)?;

        Ok(dest_path)
    }

    #[allow(dead_code)]
    pub fn store_hard_link(&self, tree_hash: &str, source_path: &Path) -> Result<PathBuf> {
        let dest_path = self.content_path(tree_hash);

        if dest_path.exists() {
            return Ok(dest_path);
        }

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).map_err(SkillmineError::Io)?;
        }

        hard_link_or_copy_dir(source_path, &dest_path)?;

        Ok(dest_path)
    }

    pub fn get(&self, tree_hash: &str) -> Option<PathBuf> {
        let path = self.content_path(tree_hash);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn link_to(&self, tree_hash: &str, target_path: &Path) -> Result<()> {
        let source_path = self.get(tree_hash).ok_or_else(|| {
            SkillmineError::Installation(format!("Content not found in store: {}", tree_hash))
        })?;

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(SkillmineError::Io)?;
        }

        match hard_link_dir(&source_path, target_path) {
            Ok(_) => Ok(()),
            Err(_) => copy_dir_all(&source_path, target_path).map_err(|e| {
                SkillmineError::Installation(format!("Failed to copy to target: {}", e))
            }),
        }
    }

    #[cfg(test)]
    pub fn stats(&self) -> Result<StoreStats> {
        let mut count = 0;
        let mut size = 0u64;

        if self.root.exists() {
            for entry in walkdir::WalkDir::new(&self.root) {
                let entry: walkdir::DirEntry = entry
                    .map_err(|e: walkdir::Error| SkillmineError::Io(std::io::Error::other(e)))?;
                if entry.file_type().is_file() {
                    count += 1;
                    size += entry
                        .metadata()
                        .map_err(|e: walkdir::Error| SkillmineError::Io(std::io::Error::other(e)))?
                        .len();
                }
            }
        }

        Ok(StoreStats { count, size })
    }
}

#[derive(Debug)]
#[cfg(test)]
pub struct StoreStats {
    pub count: u64,
    pub size: u64,
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
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
fn hard_link_or_copy_dir(src: &Path, dst: &Path) -> Result<()> {
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
fn hard_link_dir(src: &Path, dst: &Path) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_content_path() {
        let store = ContentStore::default();
        let path = store.content_path("abc123def456");
        assert!(path.to_string_lossy().contains("ab/c123def456"));
    }

    #[test]
    fn test_store_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let store = ContentStore::new(temp_dir.path().join("store"));
        store.init().unwrap();

        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        let mut file = fs::File::create(source_dir.join("test.txt")).unwrap();
        file.write_all(b"test content").unwrap();

        let hash = "abc123";
        let stored_path = store.store(hash, &source_dir).unwrap();

        assert!(stored_path.exists());
        assert!(stored_path.join("test.txt").exists());

        let retrieved = store.get(hash);
        assert_eq!(retrieved, Some(stored_path));
    }

    #[test]
    fn test_has_content_and_stats() {
        let temp_dir = TempDir::new().unwrap();
        let store = ContentStore::new(temp_dir.path().join("store"));
        store.init().unwrap();

        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        let mut file = fs::File::create(source_dir.join("test.txt")).unwrap();
        file.write_all(b"test content").unwrap();

        let hash = "abc123";
        store.store(hash, &source_dir).unwrap();

        assert!(store.has_content(hash));
        let stats = store.stats().unwrap();
        assert!(stats.count >= 1);
        assert!(stats.size > 0);
    }
}
