use anyhow::{Context, Result};
use lofty::prelude::*;
use lofty::probe::Probe;
use std::path::Path;

#[derive(Debug)]
pub struct SongMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_seconds: u64,
}

pub fn extract_metadata(path: &Path) -> Result<SongMetadata> {
    let tagged_file = Probe::open(path)
        .context("Failed to open file for probing")?
        .read()
        .context("Failed to read file tags")?;

    let tag = tagged_file.primary_tag();
    let properties = tagged_file.properties();

    let title = tag.and_then(|t| t.title().map(|s| s.to_string()));
    let artist = tag.and_then(|t| t.artist().map(|s| s.to_string()));
    let album = tag.and_then(|t| t.album().map(|s| s.to_string()));
    
    let duration_seconds = properties.duration().as_secs();

    Ok(SongMetadata {
        title,
        artist,
        album,
        duration_seconds,
    })
}
