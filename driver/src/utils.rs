use dirs::home_dir;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn expand_tilde(path: &Path) -> PathBuf {
    if !path.starts_with("~") {
        return path.to_path_buf();
    }

    if let Some(home) = home_dir() {
        if path == Path::new("~") {
            return home;
        } else {
            return home.join(path.strip_prefix("~").unwrap());
        }
    }

    path.to_path_buf()
}
