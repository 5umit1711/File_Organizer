
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoveRecord {
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub moves: Vec<MoveRecord>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            version: 1,
            moves: Vec::new(),
        }
    }
}

pub fn manifest_path(root: &Path) -> PathBuf {
    root.join("Organized")
        .join(".organizer_manifest.json")
}

pub fn load_manifest(root: &Path) -> io::Result<Manifest> {
    let path = manifest_path(root);

    match fs::read_to_string(path) {
        Ok(contents) => {
            serde_json::from_str(&contents).map_err(|error| {
                io::Error::new(io::ErrorKind::InvalidData, error)
            })
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            Ok(Manifest::default())
        }
        Err(error) => Err(error),
    }
}

pub fn save_manifest(root: &Path, manifest: &Manifest) -> io::Result<()> {
    let path = manifest_path(root);

    fs::create_dir_all(
        path.parent().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid manifest path",
            )
        })?,
    )?;

    let contents = serde_json::to_string_pretty(manifest)
        .map_err(io::Error::other)?;

    fs::write(path, contents)
}
