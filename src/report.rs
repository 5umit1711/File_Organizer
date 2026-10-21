use std::collections::HashMap;

use crate::models::{FileCategory, FileEntry};

pub fn print_report(files: &[FileEntry]) {
    let mut counts: HashMap<FileCategory, usize> = HashMap::new();
    let mut sizes: HashMap<FileCategory, u64> = HashMap::new();

    let mut total_size: u64 = 0;

    for file in files {
        *counts.entry(file.category).or_insert(0) += 1;

        *sizes.entry(file.category).or_insert(0) += file.size;

        total_size += file.size;
    }

    let categories = [
        FileCategory::Image,
        FileCategory::Document,
        FileCategory::Video,
        FileCategory::Audio,
        FileCategory::Archive,
        FileCategory::Code,
        FileCategory::Other,
    ];

    println!("\n========== STORAGE SUMMARY ==========\n");

    for category in categories {
        let count = counts.get(&category).copied().unwrap_or(0);
        let size = sizes.get(&category).copied().unwrap_or(0);

        println!(
            "{:<10} {:>6} files | {:>12}",
            category.as_str(),
            count,
            format_size(size)
        );
    }

    println!("\n-------------------------------------");
    println!("Total files: {}", files.len());
    println!("Total size:  {}", format_size(total_size));
    println!("\nScan complete. No files were modified.");
}

fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes = bytes as f64;

    if bytes >= GB {
        format!("{:.2} GB", bytes / GB)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes / KB)
    } else {
        format!("{} B", bytes as u64)
    }
}
