
use std::env;
use std::io;
use std::path::PathBuf;

pub struct AppArgs {
    pub directory: PathBuf,
    pub apply: bool,
    pub undo: bool,
}

pub fn get_args() -> io::Result<AppArgs> {
    let mut directory = None;
    let mut apply = false;
    let mut undo = false;

    for argument in env::args_os().skip(1) {
        if argument == "--apply" {
            apply = true;
        } else if argument == "--undo" {
            undo = true;
        } else if directory.is_none() {
            directory = Some(PathBuf::from(argument));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Usage: intelligent-file-organizer [DIRECTORY] [--apply | --undo]",
            ));
        }
    }

    if apply && undo {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Use --apply or --undo, not both.",
        ));
    }

    let directory = match directory {
        Some(path) => path,
        None => env::current_dir()?,
    };

    Ok(AppArgs {
        directory,
        apply,
        undo,
    })
}
