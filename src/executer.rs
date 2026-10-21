
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::manifest::{self, MoveRecord};
use crate::planner::OrganizationPlan;

pub fn execute_plan(
    root: &Path,
    plan: &OrganizationPlan,
) -> io::Result<()> {
    if plan.moves.is_empty() {
        println!("No files to move.");
        return Ok(());
    }

    println!("\nWARNING: This will move files on your computer.");
    println!("Files planned to move: {}", plan.moves.len());
    println!("Conflicts skipped: {}", plan.conflicts.len());

    print!("Type YES to continue: ");
    io::stdout().flush()?;

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;

    if confirmation.trim() != "YES" {
        println!("Cancelled. No files were moved.");
        return Ok(());
    }

    let mut history = manifest::load_manifest(root)?;
    let mut moved = 0;
    let mut failed = 0;

    for item in &plan.moves {
        if !item.source.is_file() {
            eprintln!(
                "Skipped (source file not found): {}",
                item.source.display()
            );
            failed += 1;
            continue;
        }

        if item.destination.exists() {
            eprintln!(
                "Skipped (destination exists): {}",
                item.destination.display()
            );
            failed += 1;
            continue;
        }

        let Some(parent) = item.destination.parent() else {
            eprintln!("Skipped (invalid destination path).");
            failed += 1;
            continue;
        };

        if let Err(error) = fs::create_dir_all(parent) {
            eprintln!(
                "Could not create folder {}: {}",
                parent.display(),
                error
            );
            failed += 1;
            continue;
        }

        let record = MoveRecord {
            source: item.source.to_string_lossy().into_owned(),
            destination: item.destination.to_string_lossy().into_owned(),
        };

        if !history.moves.contains(&record) {
            history.moves.push(record);
        }

        // Save the history before attempting the move.
        if let Err(error) = manifest::save_manifest(root, &history) {
            eprintln!("Could not save undo history: {error}");
            failed += 1;
            continue;
        }

        match fs::rename(&item.source, &item.destination) {
            Ok(()) => {
                println!(
                    "Moved: {} -> {}",
                    item.source.display(),
                    item.destination.display()
                );
                moved += 1;
            }
            Err(error) => {
                eprintln!(
                    "Could not move {}: {}",
                    item.source.display(),
                    error
                );
                failed += 1;
            }
        }
    }

    println!("\n========== EXECUTION SUMMARY ==========");
    println!("Successfully moved: {moved}");
    println!("Failed or skipped:  {failed}");

    Ok(())
}
