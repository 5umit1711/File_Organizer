
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::manifest::{self, Manifest, MoveRecord};

pub fn undo_moves(root: &Path) -> io::Result<()> {
    let history = manifest::load_manifest(root)?;

    if history.moves.is_empty() {
        println!("No undo history found.");
        return Ok(());
    }

    let organized_root = root.join("Organized");
    let mut remaining: Vec<MoveRecord> = Vec::new();

    let mut restored = 0;
    let mut skipped = 0;

    println!("\n========== UNDO ORGANIZATION ==========\n");

    for record in history.moves.into_iter().rev() {
        let source = PathBuf::from(&record.source);
        let destination = PathBuf::from(&record.destination);

        // Validate recorded paths before performing filesystem operations.
        if !source.starts_with(root)
            || source.starts_with(&organized_root)
            || !destination.starts_with(&organized_root)
        {
            eprintln!(
                "Skipped unsafe history entry: {}",
                record.destination
            );
            remaining.push(record);
            skipped += 1;
            continue;
        }

        if destination.exists() && !source.exists() {
            let Some(parent) = source.parent() else {
                eprintln!("Skipped invalid original path: {}", source.display());
                remaining.push(record);
                skipped += 1;
                continue;
            };

            if let Err(error) = fs::create_dir_all(parent) {
                eprintln!(
                    "Could not recreate original folder {}: {}",
                    parent.display(),
                    error
                );
                remaining.push(record);
                skipped += 1;
                continue;
            }

            match fs::rename(&destination, &source) {
                Ok(()) => {
                    println!(
                        "Restored: {} -> {}",
                        destination.display(),
                        source.display()
                    );
                    restored += 1;
                }
                Err(error) => {
                    eprintln!(
                        "Could not restore {}: {}",
                        destination.display(),
                        error
                    );
                    remaining.push(record);
                    skipped += 1;
                }
            }
        } else if source.exists() && !destination.exists() {
            // The move never happened, or the file was already restored.
            // No restore is needed for this history entry.
        } else {
            // Both paths exist, or neither path exists. Do not guess.
            eprintln!(
                "Conflict: source or destination needs attention:\n  Source: {}\n  Destination: {}",
                source.display(),
                destination.display()
            );
            remaining.push(record);
            skipped += 1;
        }
    }

    remaining.reverse();

    if remaining.is_empty() {
        let path = manifest::manifest_path(root);

        match fs::remove_file(path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    } else {
        manifest::save_manifest(
            root,
            &Manifest {
                version: 1,
                moves: remaining,
            },
        )?;
    }

    println!("\n========== UNDO SUMMARY ==========");
    println!("Successfully restored: {restored}");
    println!("Skipped or unresolved: {skipped}");

    Ok(())
}
