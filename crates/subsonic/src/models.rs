use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsonicEnvelope<T> {
    #[serde(rename = "subsonic-response")]
    pub response: SubsonicResponse<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsonicResponse<T> {
    pub status: String,
    pub version: String,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    #[serde(rename = "serverVersion")]
    pub server_version: Option<String>,
    #[serde(rename = "openSubsonic")]
    pub open_subsonic: Option<bool>,
    pub error: Option<SubsonicError>,
    #[serde(flatten)]
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsonicError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub version: String,
    pub server_type: Option<String>,
    pub server_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Child {
    pub id: String,
    pub parent: Option<String>,
    #[serde(rename = "isDir")]
    pub is_dir: Option<bool>,
    pub title: String,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub track: Option<u32>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    #[serde(rename = "coverArt")]
    pub cover_art: Option<String>,
    pub size: Option<u64>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub suffix: Option<String>,
    pub duration: Option<u32>,
    #[serde(rename = "bitRate")]
    pub bit_rate: Option<u32>,
    pub path: Option<String>,
    #[serde(rename = "playCount")]
    pub play_count: Option<u64>,
    #[serde(rename = "discNumber")]
    pub disc_number: Option<u32>,
    pub created: Option<String>,
    pub starred: Option<String>,
    #[serde(rename = "albumId")]
    pub album_id: Option<String>,
    #[serde(rename = "artistId")]
    pub artist_id: Option<String>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    #[serde(rename = "userRating")]
    pub user_rating: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumID3 {
    pub id: String,
    pub name: String,
    pub artist: Option<String>,
    #[serde(rename = "artistId")]
    pub artist_id: Option<String>,
    #[serde(rename = "coverArt")]
    pub cover_art: Option<String>,
    #[serde(rename = "songCount")]
    pub song_count: Option<u32>,
    pub duration: Option<u32>,
    pub created: Option<String>,
    pub starred: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    #[serde(rename = "playCount")]
    pub play_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumWithSongsID3 {
    #[serde(flatten)]
    pub album: AlbumID3,
    #[serde(default, rename = "song")]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistID3 {
    pub id: String,
    pub name: String,
    #[serde(rename = "coverArt")]
    pub cover_art: Option<String>,
    #[serde(rename = "artistImageUrl")]
    pub artist_image_url: Option<String>,
    #[serde(rename = "albumCount")]
    pub album_count: Option<u32>,
    pub starred: Option<String>,
    #[serde(rename = "userRating")]
    pub user_rating: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistWithAlbumsID3 {
    #[serde(flatten)]
    pub artist: ArtistID3,
    #[serde(default, rename = "album")]
    pub album: Vec<AlbumID3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistInfo2 {
    pub biography: Option<String>,
    #[serde(rename = "musicBrainzId")]
    pub music_brainz_id: Option<String>,
    #[serde(rename = "lastFmUrl")]
    pub last_fm_url: Option<String>,
    #[serde(rename = "smallImageUrl")]
    pub small_image_url: Option<String>,
    #[serde(rename = "mediumImageUrl")]
    pub medium_image_url: Option<String>,
    #[serde(rename = "largeImageUrl")]
    pub large_image_url: Option<String>,
    #[serde(default, rename = "similarArtist")]
    pub similar_artist: Vec<ArtistID3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumInfo {
    pub notes: Option<String>,
    #[serde(rename = "musicBrainzId")]
    pub music_brainz_id: Option<String>,
    #[serde(rename = "lastFmUrl")]
    pub last_fm_url: Option<String>,
    #[serde(rename = "smallImageUrl")]
    pub small_image_url: Option<String>,
    #[serde(rename = "mediumImageUrl")]
    pub medium_image_url: Option<String>,
    #[serde(rename = "largeImageUrl")]
    pub large_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub comment: Option<String>,
    pub owner: Option<String>,
    pub public: Option<bool>,
    #[serde(rename = "songCount")]
    pub song_count: Option<u32>,
    pub duration: Option<u32>,
    pub created: Option<String>,
    pub changed: Option<String>,
    #[serde(rename = "coverArt")]
    pub cover_art: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistWithSongs {
    #[serde(flatten)]
    pub playlist: Playlist,
    #[serde(default, rename = "entry")]
    pub entry: Vec<Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    #[serde(rename = "value")]
    pub name: String,
    #[serde(rename = "songCount")]
    pub song_count: Option<u32>,
    #[serde(rename = "albumCount")]
    pub album_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult3 {
    #[serde(default, rename = "artist")]
    pub artist: Vec<ArtistID3>,
    #[serde(default, rename = "album")]
    pub album: Vec<AlbumID3>,
    #[serde(default, rename = "song")]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lyrics {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricsList {
    #[serde(default, rename = "structuredLyrics")]
    pub structured_lyrics: Vec<StructuredLyrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLyrics {
    pub lang: Option<String>,
    pub synced: Option<bool>,
    #[serde(default, rename = "line")]
    pub line: Vec<LyricsLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricsLine {
    pub start: Option<i64>, // ms
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatus {
    pub scanning: bool,
    pub count: Option<u64>,
}

// Container wrappers for Subsonic endpoint responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumListContainer {
    #[serde(rename = "albumList2")]
    pub album_list: Option<AlbumListInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumListInner {
    #[serde(default, rename = "album")]
    pub album: Vec<AlbumID3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistsContainer {
    pub artists: Option<ArtistsInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistsInner {
    #[serde(default, rename = "index")]
    pub index: Vec<ArtistIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistIndex {
    pub name: String,
    #[serde(default, rename = "artist")]
    pub artist: Vec<ArtistID3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistContainer {
    pub artist: Option<ArtistWithAlbumsID3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistInfo2Container {
    #[serde(rename = "artistInfo2")]
    pub artist_info: Option<ArtistInfo2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumContainer {
    pub album: Option<AlbumWithSongsID3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumInfoContainer {
    #[serde(rename = "albumInfo")]
    pub album_info: Option<AlbumInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistsContainer {
    pub playlists: Option<PlaylistsInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistsInner {
    #[serde(default, rename = "playlist")]
    pub playlist: Vec<Playlist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistContainer {
    pub playlist: Option<PlaylistWithSongs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenresContainer {
    pub genres: Option<GenresInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenresInner {
    #[serde(default, rename = "genre")]
    pub genre: Vec<Genre>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult3Container {
    #[serde(rename = "searchResult3")]
    pub search_result: Option<SearchResult3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongContainer {
    pub song: Option<Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarSongs2Container {
    #[serde(rename = "similarSongs2")]
    pub similar_songs: Option<SimilarSongsInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarSongsInner {
    #[serde(default, rename = "song")]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSongsContainer {
    #[serde(rename = "topSongs")]
    pub top_songs: Option<SimilarSongsInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricsListContainer {
    #[serde(rename = "lyricsList")]
    pub lyrics_list: Option<LyricsList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatusContainer {
    #[serde(rename = "scanStatus")]
    pub scan_status: Option<ScanStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Starred2 {
    #[serde(default, rename = "artist")]
    pub artist: Vec<ArtistID3>,
    #[serde(default, rename = "album")]
    pub album: Vec<AlbumID3>,
    #[serde(default, rename = "song")]
    pub song: Vec<Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Starred2Container {
    pub starred2: Option<Starred2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomSongsContainer {
    #[serde(rename = "randomSongs")]
    pub random_songs: Option<SimilarSongsInner>,
}

