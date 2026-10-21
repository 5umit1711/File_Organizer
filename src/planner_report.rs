use crate::planner::OrganizationPlan;

pub fn print_plan(plan: &OrganizationPlan) {
    println!("\n========== ORGANIZATION PLAN ==========\n");

    for item in &plan.moves {
        println!("FROM: {}", item.source.display());
        println!("TO:   {}", item.destination.display());
        println!("SIZE: {} bytes\n", item.size);
    }

    if !plan.conflicts.is_empty() {
        println!("========== CONFLICTS ==========\n");

        for conflict in &plan.conflicts {
            println!("File:   {}", conflict.source.display());
            println!("Target: {}", conflict.destination.display());
            println!("Reason: {}\n", conflict.reason);
        }
    }

    println!("=======================================");
    println!("Files planned:  {}", plan.moves.len());
    println!("Conflicts:      {}", plan.conflicts.len());
    println!("Files skipped:  {}", plan.skipped);
    println!("\nDry run complete. No files were modified.");
}