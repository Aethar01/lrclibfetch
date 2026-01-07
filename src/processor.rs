use std::path::{Path, PathBuf};
use tokio::fs;
use anyhow::{Context, Result};
use crate::api::ApiClient;
use crate::metadata::extract_metadata;

pub struct Processor {
    api_client: ApiClient,
    base_dir: PathBuf,
    output_dir: PathBuf,
}

impl Processor {
    pub fn new(base_dir: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            api_client: ApiClient::new(),
            base_dir,
            output_dir,
        }
    }

    pub async fn process_file(&self, file_path: &Path) -> Result<()> {
        let relative_path = match file_path.strip_prefix(&self.base_dir) {
            Ok(p) => p,
            Err(_) => {
                 file_path.file_name().map(Path::new).unwrap_or(file_path)
            }
        };

        let mut output_path = self.output_dir.join(relative_path);
        output_path.set_extension("lrc");

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await.context("Failed to create output directory")?;
        }

        println!("Processing: {:?}", file_path.file_name().unwrap_or_default());

        let meta = match extract_metadata(file_path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Skipping: Failed to extract metadata: {}", e);
                return Ok(());
            }
        };

        match self.api_client.fetch_lyrics(&meta).await {
            Ok(Some(lyrics)) => {
                fs::write(&output_path, lyrics).await.context("Failed to write .lrc file")?;
                println!("Saved: {:?}", output_path);
            }
            Ok(None) => {
                println!("Not found");
            }
            Err(e) => {
                eprintln!("Error fetching lyrics: {}", e);
            }
        }

        Ok(())
    }
}
