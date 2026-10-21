
mod command;
mod scanner;
mod categorizer;
mod report;
mod models;
mod planner;
mod planner_report;
mod executer;
mod manifest;
mod undo;

fn main() {
    let args = match command::get_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Argument error: {error}");
            std::process::exit(1);
        }
    };

    let directory = match std::fs::canonicalize(&args.directory) {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Could not access directory: {error}");
            std::process::exit(1);
        }
    };

    if !directory.is_dir() {
        eprintln!("The supplied path is not a directory.");
        std::process::exit(1);
    }

    // Restore previously organized files if --undo was supplied.
    if args.undo {
        if let Err(error) = undo::undo_moves(&directory) {
            eprintln!("Undo failed: {error}");
            std::process::exit(1);
        }

        return;
    }

    println!("Scanning: {}", directory.display());

    let files = match scanner::scan_directory(&directory) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("Could not scan directory: {error}");
            std::process::exit(1);
        }
    };

    report::print_report(&files);

    let plan = planner::build_plan(&directory, &files);
    planner_report::print_plan(&plan);

    if args.apply {
        if let Err(error) = executer::execute_plan(&directory, &plan) {
            eprintln!("Execution error: {error}");
            std::process::exit(1);
        }
    } else {
        println!("\nPreview mode: no files were modified.");
        println!("Use --apply to request file moves.");
    }
}
