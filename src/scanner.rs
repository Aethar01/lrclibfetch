use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_audio_files(input: &Path) -> Vec<PathBuf> {
    if input.is_file() {
        return vec![input.to_path_buf()];
    }

    let supported_extensions = ["mp3", "flac", "m4a", "ogg", "wav", "opus"];

    WalkDir::new(input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| supported_extensions.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_owned())
        .collect()
}
