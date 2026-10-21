
use std::fs;
use std::io;
use std::path::Path;

use crate::categorizer::categorize_file;
use crate::models::FileEntry;

pub fn scan_directory(directory: &Path) -> io::Result<Vec<FileEntry>> {
    if !directory.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The supplied path is not a directory",
        ));
    }

    let mut files = Vec::new();
    scan_recursive(directory, &mut files)?;

    Ok(files)
}

fn scan_recursive(
    directory: &Path,
    files: &mut Vec<FileEntry>,
) -> io::Result<()> {
    for entry_result in fs::read_dir(directory)? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;

        if file_type.is_symlink() {
            continue;
        }

        if should_skip_entry(&entry)? {
            continue;
        }

        let path = entry.path();

        if file_type.is_dir() {
            scan_recursive(&path, files)?;
        } else if file_type.is_file() {
            let metadata = entry.metadata()?;

            files.push(FileEntry {
                category: categorize_file(&path),
                path,
                size: metadata.len(),
            });
        }
    }

    Ok(())
}

fn should_skip_entry(entry: &fs::DirEntry) -> io::Result<bool> {
    let name = entry.file_name();
    let name = name.to_string_lossy();

    if name.starts_with('.') || name.eq_ignore_ascii_case("desktop.ini") {
        return Ok(true);
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;

        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
        const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;

        let metadata = entry.metadata()?;
        let attributes = metadata.file_attributes();

        if attributes & (FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM) != 0 {
            return Ok(true);
        }
    }

    Ok(false)
}
