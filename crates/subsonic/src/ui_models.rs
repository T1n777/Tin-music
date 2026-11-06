use serde::{Deserialize, Serialize};

/// How the signed-in user rated a track.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    Like,
    Dislike,
    Indifferent,
}

/// One run of an artist line: its text, plus an id when linked.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtistRun {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Segment {
    pub start_ms: i64,
    pub twin_start_ms: i64,
    pub duration_ms: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LibraryToggle {
    pub in_library: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_token: Option<String>,
}

pub type SongLibraryState = LibraryToggle;

/// One song item — standard wire type for queue, track rows, playback.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SongItem {
    pub video_id: String,
    pub title: String,
    pub artists: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artist_runs: Vec<ArtistRun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub album_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub play_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_video_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_by_avatars: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<Rating>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_by: Option<String>,
    #[serde(default)]
    pub queued: bool,
    #[serde(default)]
    pub queued_end: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_from: Option<String>,
    #[serde(default)]
    pub autoplay: bool,
    #[serde(default)]
    pub is_video: bool,
    #[serde(default)]
    pub is_episode: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counterpart: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counterpart_thumbnail: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub counterpart_segments: Vec<Segment>,
    #[serde(default)]
    pub is_upload: bool,
    #[serde(default)]
    pub explicit: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub library: Option<LibraryToggle>,
}

/// One clickable card in a home carousel or library grid.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BrowseItem {
    /// `song` | `playlist` | `album` | `artist`.
    pub kind: String,
    /// ID (song id, album id, artist id, playlist id).
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artist_runs: Vec<ArtistRun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<String>,
    #[serde(default)]
    pub is_video: bool,
    #[serde(default)]
    pub is_upload: bool,
    #[serde(default)]
    pub explicit: bool,
}

/// Titled carousel / shelf on Home or Artist pages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    pub items: Vec<BrowseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HomeChip {
    pub title: String,
    pub params: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct HomePage {
    #[serde(default)]
    pub chips: Vec<HomeChip>,
    pub sections: Vec<Section>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPage {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub artist_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artist_runs: Vec<ArtistRun>,
    pub artist_thumbnail: Option<String>,
    pub subtitle: Option<String>,
    pub second_subtitle: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub items: Vec<SongItem>,
    pub continuation: Option<String>,
    pub explicit: bool,
    pub playlist_id: Option<String>,
    pub in_library: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<ArtistCarousel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArtistPage {
    pub name: Option<String>,
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub subscribers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_listeners: Option<String>,
    pub channel_id: String,
    pub subscribed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radio_playlist_id: Option<String>,
    pub top_songs: Vec<SongItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_songs_id: Option<String>,
    pub sections: Vec<ArtistCarousel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArtistCarousel {
    pub title: String,
    pub items: Vec<BrowseItem>,
    pub more_browse_id: Option<String>,
    pub more_params: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPage {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
    pub privacy: Option<String>,
    pub cover: Option<String>,
    pub items: Vec<SongItem>,
    pub continuation: Option<String>,
    pub owned: bool,
    pub collaborative: bool,
    pub sort_menu: Option<SortMenu>,
    pub suggestions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SortMenu {
    pub editable: bool,
    pub current: PlaylistSort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaylistSort {
    Default,
    Newest,
    Oldest,
    Title,
    Artist,
    Album,
    Top,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResults {
    pub songs: Vec<SongItem>,
    pub albums: Vec<BrowseItem>,
    pub artists: Vec<BrowseItem>,
    pub playlists: Vec<BrowseItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchSuggestions {
    pub queries: Vec<QuerySuggestion>,
    pub items: Vec<BrowseItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuerySuggestion {
    pub text: String,
    pub history: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryGroup {
    pub title: String,
    pub items: Vec<SongItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimedLyricLine {
    pub text: String,
    pub start_ms: i64,
    pub end_ms: Option<i64>,
}
