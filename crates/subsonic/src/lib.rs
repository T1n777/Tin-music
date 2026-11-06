//! Pure-Rust Subsonic & OpenSubsonic API client for Navidrome.

pub mod adapter;
pub mod client;
pub mod models;
pub mod ui_models;

pub use client::{Error, SubsonicClient};
pub use models::ServerInfo;
pub use adapter::{
    album_to_browse_item, artist_to_browse_item, child_to_song_item, playlist_to_browse_item,
};
pub use ui_models::{
    AlbumPage, ArtistCarousel, ArtistPage, ArtistRun, BrowseItem, HistoryGroup, HomeChip,
    HomePage, PlaylistPage, PlaylistSort, QuerySuggestion, Rating, SearchResults,
    SearchSuggestions, Section, SongItem, SongLibraryState, SortMenu, TimedLyricLine,
};
