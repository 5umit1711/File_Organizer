
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::models::FileEntry;

#[derive(Debug)]
pub struct PlannedMove {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub size: u64,
}

#[derive(Debug)]
pub struct PlanConflict {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub reason: &'static str,
}

#[derive(Debug, Default)]
pub struct OrganizationPlan {
    pub moves: Vec<PlannedMove>,
    pub conflicts: Vec<PlanConflict>,
    pub skipped: usize,
}

pub fn build_plan(root: &Path, files: &[FileEntry]) -> OrganizationPlan {
    let organized_root = root.join("Organized");

    let mut plan = OrganizationPlan::default();
    let mut claimed_destinations: HashSet<String> = HashSet::new();

    for file in files {
        if file.path.starts_with(&organized_root) {
            plan.skipped += 1;
            continue;
        }

        let Some(file_name) = file.path.file_name() else {
            plan.skipped += 1;
            continue;
        };

        let destination = organized_root
            .join(file.category.folder_name())
            .join(file_name);

        if destination.exists() {
            plan.conflicts.push(PlanConflict {
                source: file.path.clone(),
                destination,
                reason: "Destination already exists",
            });

            continue;
        }

        let key = collision_key(&destination);

        if !claimed_destinations.insert(key) {
            plan.conflicts.push(PlanConflict {
                source: file.path.clone(),
                destination,
                reason: "Another file has claimed this destination",
            });

            continue;
        }

        plan.moves.push(PlannedMove {
            source: file.path.clone(),
            destination,
            size: file.size,
        });
    }

    plan
}

fn collision_key(path: &Path) -> String {
    let path_string = path.to_string_lossy().into_owned();

    if cfg!(windows) {
        path_string.to_lowercase()
    } else {
        path_string
    }
}
