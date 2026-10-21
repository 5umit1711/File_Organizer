use std::path::Path;

use crate::models::FileCategory;

pub fn categorize_file(path: &Path) -> FileCategory {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match extension.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" => FileCategory::Image,

        "pdf" | "doc" | "docx" | "txt" | "odt" => FileCategory::Document,

        "mp4" | "mkv" | "avi" | "mov" | "webm" => FileCategory::Video,

        "mp3" | "wav" | "flac" | "aac" | "ogg" => FileCategory::Audio,

        "zip" | "rar" | "7z" | "tar" | "gz" => FileCategory::Archive,

        "rs" | "cpp" | "c" | "h" | "py" | "js" | "ts" | "html" | "css" | "java" => {
            FileCategory::Code
        }

        _ => FileCategory::Other,
    }
}
