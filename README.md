# Intelligent File Organizer

A command-line file organizer written in Rust that scans directories,
categorizes files by extension, summarizes storage usage, and proposes a
cleaner folder structure. It supports previewing changes before applying
them and records file moves so supported operations can be undone later.

> **Safety note:** Preview mode does not move files. Use `--apply` only
> after reviewing the plan. Test on a disposable folder before using it
> on important files.

## Features

-   **Recursive directory scanning** --- scans files inside the selected
    directory and its subdirectories.
-   **Extension-based categorization** --- groups images, documents,
    videos, audio, archives, source code, and other files.
-   **Storage summary** --- reports file counts and total sizes by
    category.
-   **Dry-run preview** --- shows proposed source and destination paths
    without changing files.
-   **Conflict checks** --- flags destinations that already exist and
    duplicate planned destinations.
-   **Explicit confirmation** --- requires the user to type `YES` before
    applying a plan.
-   **Move history and undo** --- stores move records in a JSON manifest
    and can restore files to their recorded original paths.
-   **Hidden/system entry filtering** --- skips dot-prefixed entries,
    `desktop.ini`, and Windows entries marked Hidden or System.
-   **Modular codebase** --- separates scanning, categorization,
    planning, reporting, execution, and undo logic.

## Technologies Used

-   **Rust** --- application logic and command-line program.
-   **Cargo** --- build, dependency, formatting, and run workflow.
-   **Serde** --- serializes and deserializes move-history data.
-   **Serde JSON** --- stores the move manifest as JSON.
-   **Rust standard library** --- filesystem access, paths, command-line
    arguments, and error handling.

## Project Structure

``` text
intelligent-file-organizer/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── command.rs
    ├── scanner.rs
    ├── categorizer.rs
    ├── report.rs
    ├── models.rs
    ├── planner.rs
    ├── planner_report.rs
    ├── executor.rs
    ├── manifest.rs
    └── undo.rs
```

  -----------------------------------------------------------------------
  File                                Responsibility
  ----------------------------------- -----------------------------------
  `main.rs`                           Connects command parsing, scanning,
                                      reporting, planning, execution, and
                                      undo.

  `command.rs`                        Parses the directory argument and
                                      `--apply` / `--undo` options.

  `scanner.rs`                        Recursively scans directories and
                                      filters selected hidden/system
                                      entries.

  `categorizer.rs`                    Categorizes files using their
                                      extensions.

  `models.rs`                         Defines file categories and scanned
                                      file data.

  `report.rs`                         Prints storage usage statistics.

  `planner.rs`                        Builds the proposed organization
                                      plan and checks destination
                                      conflicts.

  `planner_report.rs`                 Prints the proposed moves and
                                      conflicts.

  `executor.rs`                       Requests confirmation and attempts
                                      to move planned files.

  `manifest.rs`                       Reads and writes the JSON
                                      move-history manifest.

  `undo.rs`                           Restores files based on recorded
                                      move history.
  -----------------------------------------------------------------------

## Requirements

-   Install the [Rust
    toolchain](https://www.rust-lang.org/tools/install), including
    `rustc` and `cargo`.
-   A terminal such as PowerShell, Command Prompt, or a Unix shell.

Check your installation:

``` bash
rustc --version
cargo --version
```

## Setup

1.  Clone the repository:

    ``` bash
    git clone <YOUR-REPOSITORY-URL>
    cd intelligent-file-organizer
    ```

    Replace `<YOUR-REPOSITORY-URL>` with your GitHub repository URL.

2.  Make sure `Cargo.toml` includes the dependencies below. If they are
    missing, add them from the project directory:

    ``` bash
    cargo add serde --features derive
    cargo add serde_json
    ```

3.  Format and check the project:

    ``` bash
    cargo fmt
    cargo check
    ```

## Usage

Run all commands from the project directory, where `Cargo.toml` is
located.

### 1. Preview a directory (default)

The default mode scans and reports proposed destinations but does not
move files.

**PowerShell:**

``` powershell
cargo run -- "$env:USERPROFILE\Downloads"
```

**Example with a relative test directory:**

``` powershell
cargo run -- .\test-files
```

**Other shells:**

``` bash
cargo run -- "$HOME/Downloads"
```

The plan proposes folders such as `Organized/Images`,
`Organized/Documents`, `Organized/Videos`, `Organized/Audio`,
`Organized/Archives`, `Organized/Code`, and `Organized/Others` under the
selected directory.

### 2. Apply the plan

Review the preview first. To request file moves, add `--apply`:

**PowerShell:**

``` powershell
cargo run -- "$env:USERPROFILE\Downloads" --apply
```

The program asks for confirmation. Type `YES` only if the proposed plan
is correct. Files are moved, not copied. If a destination exists, the
executor skips that move rather than intentionally overwriting the
destination.

**Always test with disposable files first.** Filesystem operations can
fail, and cross-drive moves may not work with the current `fs::rename`
implementation.

### 3. Undo recorded moves

To restore files recorded in the manifest:

**PowerShell:**

``` powershell
cargo run -- "$env:USERPROFILE\Downloads" --undo
```

**Test directory:**

``` powershell
cargo run -- .\test-files --undo
```

Undo uses the move history stored at:

``` text
<selected-directory>\Organized\.organizer_manifest.json
```

The manifest is important: do not delete it if you need to undo the
recorded moves. Undo is intended for operations recorded by this version
of the application; files moved before history tracking was added cannot
be reliably restored automatically. If an original path is occupied or
the recorded paths are inconsistent, the program skips the entry and
retains unresolved history instead of guessing.

### 4. Create a safe test directory

You can try the project without using your real Downloads folder:

``` powershell
New-Item -ItemType Directory -Force test-files\docs
Set-Content test-files\docs\notes.txt "Test document"
Set-Content test-files\photo.jpg "Test image"
```

Preview:

``` powershell
cargo run -- .\test-files
```

Apply after checking the preview:

``` powershell
cargo run -- .\test-files --apply
```

Undo:

``` powershell
cargo run -- .\test-files --undo
```

## Current Limitations

-   Categorization is based on file extensions, not file-content
    detection or AI.
-   File moves use `std::fs::rename`; moving across different
    drives/volumes may fail.
-   The undo feature depends on the JSON manifest and only knows about
    recorded moves.
-   The current planner and manifest are designed as a practical
    learning project; additional hardening and automated tests are
    recommended before relying on it for large or important directories.
-   Empty folders created during organization may remain after undo.

## Roadmap

-   Add automated unit and integration tests.
-   Improve cross-drive move handling.
-   Improve crash recovery and manifest consistency.
-   Add configurable categories and custom organization rules.
-   Add duplicate-file detection using file hashes.
-   Add richer storage analytics, such as largest files and age-based
    filtering.

## Contributing

Issues and pull requests are welcome. For changes involving file moves
or undo, please include tests that cover existing destinations,
duplicate names, failed operations, and interrupted workflows.

## License

Choose and add a license file (for example, MIT or Apache-2.0) before
distributing the project. Do not claim a license until one is added to
the repository.
