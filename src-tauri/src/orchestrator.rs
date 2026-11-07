//! Orchestrator: videoId → a playable stream URL via Navidrome (Subsonic API).

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

pub fn is_youtube_stream(_url: &str) -> bool {
    false
}

/// Everything the player + UI + media layer need for one track.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlaybackData {
    pub video_id: String,
    pub stream_url: String,
    pub itag: i64,
    /// HTTP headers mpv must send.
    #[serde(skip)]
    pub headers: HashMap<String, String>,
    pub expires_in_seconds: i64,
    pub loudness_db: Option<f64>,
    pub playback_ping: Option<PlaybackPing>,
    pub title: Option<String>,
    pub artists: Option<String>,
    pub duration: Option<String>,
    pub thumbnail: Option<String>,
    pub is_video: Option<bool>,
    pub is_episode: Option<bool>,
    pub stream_client: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PlaybackPing {
    pub url: String,
    pub client: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("Could not resolve stream for {0}")]
    AllClientsFailed(String),
    #[error("This track could not be played ({0})")]
    UploadUnavailable(String),
    #[error("Navidrome server connection required. Connect from Settings.")]
    SignInRequired(String),
    #[error("This file is no longer on your disk: {0}")]
    LocalMissing(String),
    #[error("Could not reach Navidrome server ({0})")]
    Unreachable(String),
}

impl ResolveError {
    pub fn affects_every_track(&self) -> bool {
        matches!(self, ResolveError::Unreachable(_) | ResolveError::SignInRequired(_))
    }
}

pub struct Orchestrator {
    subsonic: Arc<RwLock<Option<subsonic::SubsonicClient>>>,
}

impl Orchestrator {
    pub fn new(subsonic: Arc<RwLock<Option<subsonic::SubsonicClient>>>) -> Self {
        Self { subsonic }
    }

    pub fn mark_video_failed(&self, _video_id: &str) -> bool {
        false
    }

    pub async fn mark_web_remix_failed(&self, _video_id: &str) {}

    pub fn headers_for(&self, _client: &str, _is_upload: bool) -> HashMap<String, String> {
        HashMap::new()
    }

    pub async fn resolve(
        &self,
        video_id: &str,
        _is_upload: bool,
        max_bitrate: Option<u32>,
        _disabled: &HashSet<String>,
    ) -> Result<PlaybackData, ResolveError> {
        let client = {
            let guard = self.subsonic.read().await;
            guard.clone()
        };

        let Some(client) = client else {
            return Err(ResolveError::SignInRequired("Navidrome server not connected".to_string()));
        };

        let stream_url = client.stream_url(video_id, max_bitrate);

        Ok(PlaybackData {
            video_id: video_id.to_string(),
            stream_url,
            itag: 0,
            headers: HashMap::new(),
            expires_in_seconds: 3600 * 24,
            loudness_db: None,
            playback_ping: None,
            title: None,
            artists: None,
            duration: None,
            thumbnail: None,
            is_video: Some(false),
            is_episode: Some(false),
            stream_client: "navidrome".to_string(),
        })
    }

    pub async fn resolve_video(&self, _video_id: &str, _max_height: i32, _disabled: &HashSet<String>) -> Option<String> {
        None
    }
}
