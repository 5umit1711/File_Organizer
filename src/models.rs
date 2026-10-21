use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileCategory {
    Image,
    Document,
    Video,
    Audio,
    Archive,
    Code,
    Other,
}

impl FileCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Image => "Image",
            Self::Document => "Document",
            Self::Video => "Video",
            Self::Audio => "Audio",
            Self::Archive => "Archive",
            Self::Code => "Code",
            Self::Other => "Other",
        }
    }

    pub fn folder_name(&self) -> &'static str {
        match self {
            Self::Image => "Images",
            Self::Document => "Documents",
            Self::Video => "Videos",
            Self::Audio => "Audio",
            Self::Archive => "Archives",
            Self::Code => "Code",
            Self::Other => "Others",
        }
    }
}

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub category: FileCategory,
}
