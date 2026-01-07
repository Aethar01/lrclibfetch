use anyhow::{Result, bail};
use reqwest::Client;
use serde::Deserialize;
use crate::metadata::SongMetadata;

const BASE_URL: &str = "https://lrclib.net/api";

#[derive(Debug, Deserialize)]
pub struct LrcLibResponse {
    #[serde(rename = "syncedLyrics")]
    pub synced_lyrics: Option<String>,
    #[serde(rename = "plainLyrics")]
    pub plain_lyrics: Option<String>,
}

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_lyrics(&self, meta: &SongMetadata) -> Result<Option<String>> {
        let title = meta.title.as_deref().unwrap_or("");
        let artist = meta.artist.as_deref().unwrap_or("");
        let album = meta.album.as_deref().unwrap_or("");
        
        if title.is_empty() {
             bail!("Song title is missing, cannot search.");
        }

        let mut query = vec![
            ("track_name", title),
            ("artist_name", artist),
            ("album_name", album),
        ];
        
        let duration_str = meta.duration_seconds.to_string();
        query.push(("duration", &duration_str));

        let response = self.client.get(format!("{}/get", BASE_URL))
            .query(&query)
            .send()
            .await?;

        if response.status().is_success() {
             let data: LrcLibResponse = response.json().await?;
             if let Some(lyrics) = data.synced_lyrics {
                 return Ok(Some(lyrics));
             } else if let Some(lyrics) = data.plain_lyrics {
                 println!("Warning: only plain lyrics found.");
                 return Ok(Some(lyrics));
             }
        }

        Ok(None)
    }
}
