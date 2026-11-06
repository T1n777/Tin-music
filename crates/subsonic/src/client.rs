use std::time::Duration;
use md5::{Digest, Md5};
use reqwest::Client;
use url::Url;

use crate::adapter::{
    album_to_browse_item, album_with_songs_to_page, artist_to_browse_item,
    artist_with_albums_to_page, child_to_song_item, playlist_to_browse_item,
    playlist_with_songs_to_page,
};
use crate::models::*;
use crate::ui_models::{
    AlbumPage, ArtistPage, BrowseItem, HomeChip, HomePage, PlaylistPage,
    SearchResults, Section, SongItem, TimedLyricLine,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("Subsonic error ({code}): {message}")]
    Subsonic { code: i32, message: String },
    #[error("Missing data in Subsonic response")]
    MissingData,
    #[error("Connection error: {0}")]
    Connection(String),
}

#[derive(Clone)]
pub struct SubsonicClient {
    base_url: Url,
    username: String,
    token: String,
    salt: String,
    client_name: String,
    api_version: String,
    http: Client,
}

impl SubsonicClient {
    /// Create a new Subsonic client using password. Generates salt and md5(password + salt) token.
    pub fn new(base_url_str: &str, username: &str, password: &str) -> Result<Self, Error> {
        let base_url = Url::parse(base_url_str)?;
        let salt = generate_salt();
        let token = compute_token(password, &salt);

        let http = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()?;

        Ok(Self {
            base_url,
            username: username.to_string(),
            token,
            salt,
            client_name: "Limusic".to_string(),
            api_version: "1.16.1".to_string(),
            http,
        })
    }

    /// Create from already-persisted token and salt.
    pub fn from_token(base_url_str: &str, username: &str, token: &str, salt: &str) -> Result<Self, Error> {
        let base_url = Url::parse(base_url_str)?;
        let http = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()?;

        Ok(Self {
            base_url,
            username: username.to_string(),
            token: token.to_string(),
            salt: salt.to_string(),
            client_name: "Limusic".to_string(),
            api_version: "1.16.1".to_string(),
            http,
        })
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn salt(&self) -> &str {
        &self.salt
    }

    fn endpoint_url(&self, action: &str) -> Result<Url, Error> {
        let mut path = self.base_url.path().trim_end_matches('/').to_string();
        if !path.ends_with("/rest") {
            path.push_str("/rest");
        }
        path.push('/');
        path.push_str(action);
        if !path.ends_with(".view") {
            path.push_str(".view");
        }

        let mut url = self.base_url.clone();
        url.set_path(&path);

        url.query_pairs_mut()
            .append_pair("u", &self.username)
            .append_pair("t", &self.token)
            .append_pair("s", &self.salt)
            .append_pair("v", &self.api_version)
            .append_pair("c", &self.client_name)
            .append_pair("f", "json");

        Ok(url)
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, action: &str, params: &[(&str, &str)]) -> Result<T, Error> {
        let mut url = self.endpoint_url(action)?;
        {
            let mut q = url.query_pairs_mut();
            for (k, v) in params {
                q.append_pair(k, v);
            }
        }

        let res = self.http.get(url).send().await?;
        let envelope: SubsonicEnvelope<T> = res.json().await?;

        if let Some(err) = envelope.response.error {
            return Err(Error::Subsonic {
                code: err.code,
                message: err.message,
            });
        }

        envelope.response.data.ok_or(Error::MissingData)
    }

    pub fn cover_art_url(&self, id: &str, size: Option<u32>) -> String {
        let Ok(mut url) = self.endpoint_url("getCoverArt") else {
            return String::new();
        };
        {
            let mut q = url.query_pairs_mut();
            q.append_pair("id", id);
            if let Some(s) = size {
                q.append_pair("size", &s.to_string());
            }
        }
        url.to_string()
    }

    pub fn stream_url(&self, id: &str, max_bit_rate: Option<u32>) -> String {
        let Ok(mut url) = self.endpoint_url("stream") else {
            return String::new();
        };
        {
            let mut q = url.query_pairs_mut();
            q.append_pair("id", id);
            if let Some(mbr) = max_bit_rate {
                if mbr > 0 {
                    q.append_pair("maxBitRate", &mbr.to_string());
                }
            }
        }
        url.to_string()
    }

    // --- Endpoints ---

    pub async fn ping(&self) -> Result<ServerInfo, Error> {
        let url = self.endpoint_url("ping")?;
        let res = self.http.get(url).send().await?;
        let envelope: SubsonicEnvelope<serde_json::Value> = res.json().await?;

        if let Some(err) = envelope.response.error {
            return Err(Error::Subsonic {
                code: err.code,
                message: err.message,
            });
        }

        Ok(ServerInfo {
            version: envelope.response.version,
            server_type: envelope.response.server_type,
            server_version: envelope.response.server_version,
        })
    }

    pub async fn get_album_list(&self, list_type: &str, size: u32, offset: u32) -> Result<Vec<BrowseItem>, Error> {
        let data: AlbumListContainer = self
            .get_json(
                "getAlbumList2",
                &[
                    ("type", list_type),
                    ("size", &size.to_string()),
                    ("offset", &offset.to_string()),
                ],
            )
            .await?;

        let albums = data.album_list.map(|l| l.album).unwrap_or_default();
        Ok(albums
            .into_iter()
            .map(|a| {
                let cover = self.cover_art_url(&a.cover_art.as_deref().unwrap_or(&a.id), Some(400));
                album_to_browse_item(&a, Some(cover))
            })
            .collect())
    }

    pub async fn get_home(&self) -> Result<HomePage, Error> {
        self.get_home_with_params(None).await
    }

    pub async fn get_home_with_params(&self, params: Option<&str>) -> Result<HomePage, Error> {
        let chips = vec![
            HomeChip {
                title: "Recently Added".to_string(),
                params: "newest".to_string(),
            },
            HomeChip {
                title: "Recently Played".to_string(),
                params: "recent".to_string(),
            },
            HomeChip {
                title: "Most Played".to_string(),
                params: "frequent".to_string(),
            },
            HomeChip {
                title: "Favorites".to_string(),
                params: "starred".to_string(),
            },
            HomeChip {
                title: "Random".to_string(),
                params: "random".to_string(),
            },
        ];

        match params {
            Some("recent") => {
                let items = self.get_album_list("recent", 36, 0).await.unwrap_or_default();
                let cont = if items.len() >= 36 { Some("offset:36:kind:recent".to_string()) } else { None };
                Ok(HomePage {
                    chips,
                    sections: vec![Section {
                        title: "Recently Played".to_string(),
                        subtitle: Some("Jump back in".to_string()),
                        items,
                        kind: Some("recent".to_string()),
                    }],
                    continuation: cont,
                })
            }
            Some("frequent") => {
                let items = self.get_album_list("frequent", 36, 0).await.unwrap_or_default();
                let cont = if items.len() >= 36 { Some("offset:36:kind:frequent".to_string()) } else { None };
                Ok(HomePage {
                    chips,
                    sections: vec![Section {
                        title: "Most Played".to_string(),
                        subtitle: Some("Your most listened albums".to_string()),
                        items,
                        kind: Some("frequent".to_string()),
                    }],
                    continuation: cont,
                })
            }
            Some("starred") => {
                let items = self.get_album_list("starred", 36, 0).await.unwrap_or_default();
                let cont = if items.len() >= 36 { Some("offset:36:kind:starred".to_string()) } else { None };
                Ok(HomePage {
                    chips,
                    sections: vec![Section {
                        title: "Favorite Albums".to_string(),
                        subtitle: Some("Starred in Navidrome".to_string()),
                        items,
                        kind: Some("starred".to_string()),
                    }],
                    continuation: cont,
                })
            }
            Some("newest") => {
                let items = self.get_album_list("newest", 36, 0).await.unwrap_or_default();
                let cont = if items.len() >= 36 { Some("offset:36:kind:newest".to_string()) } else { None };
                Ok(HomePage {
                    chips,
                    sections: vec![Section {
                        title: "Recently Added".to_string(),
                        subtitle: Some("Newest additions to your collection".to_string()),
                        items,
                        kind: Some("newest".to_string()),
                    }],
                    continuation: cont,
                })
            }
            Some("random") => {
                let items = self.get_album_list("random", 36, 0).await.unwrap_or_default();
                Ok(HomePage {
                    chips,
                    sections: vec![Section {
                        title: "Random Mix".to_string(),
                        subtitle: Some("Rediscover your collection".to_string()),
                        items,
                        kind: Some("random".to_string()),
                    }],
                    continuation: Some("offset:36:kind:random".to_string()),
                })
            }
            _ => {
                let (newest, frequent, recent, random, starred) = tokio::join!(
                    self.get_album_list("newest", 12, 0),
                    self.get_album_list("frequent", 12, 0),
                    self.get_album_list("recent", 12, 0),
                    self.get_album_list("random", 12, 0),
                    self.get_album_list("starred", 12, 0),
                );

                let mut sections = Vec::new();

                if let Ok(items) = newest {
                    if !items.is_empty() {
                        sections.push(Section {
                            title: "Recently Added".to_string(),
                            subtitle: Some("Newest additions to your Navidrome library".to_string()),
                            items,
                            kind: Some("newest".to_string()),
                        });
                    }
                }

                if let Ok(items) = frequent {
                    if !items.is_empty() {
                        sections.push(Section {
                            title: "Most Played".to_string(),
                            subtitle: Some("Your most listened albums".to_string()),
                            items,
                            kind: Some("frequent".to_string()),
                        });
                    }
                }

                if let Ok(items) = recent {
                    if !items.is_empty() {
                        sections.push(Section {
                            title: "Recently Played".to_string(),
                            subtitle: Some("Jump back in".to_string()),
                            items,
                            kind: Some("recent".to_string()),
                        });
                    }
                }

                if let Ok(items) = random {
                    if !items.is_empty() {
                        sections.push(Section {
                            title: "Random Mix".to_string(),
                            subtitle: Some("Rediscover your collection".to_string()),
                            items,
                            kind: Some("random".to_string()),
                        });
                    }
                }

                if let Ok(items) = starred {
                    if !items.is_empty() {
                        sections.push(Section {
                            title: "Favorite Albums".to_string(),
                            subtitle: Some("Starred in Navidrome".to_string()),
                            items,
                            kind: Some("starred".to_string()),
                        });
                    }
                }

                if sections.len() < 3 {
                    if let Ok(explore) = self.get_album_list("alphabeticalByName", 12, 0).await {
                        if !explore.is_empty() {
                            sections.push(Section {
                                title: "Explore Collection".to_string(),
                                subtitle: Some("Browse your Navidrome albums".to_string()),
                                items: explore,
                                kind: Some("explore".to_string()),
                            });
                        }
                    }
                }

                Ok(HomePage {
                    chips,
                    sections,
                    continuation: Some("offset:12:kind:all".to_string()),
                })
            }
        }
    }

    pub async fn get_home_more(&self, token: &str) -> Result<HomePage, Error> {
        let mut offset = 12u32;
        let mut kind = "newest".to_string();

        let parts: Vec<&str> = token.split(':').collect();
        for i in 0..parts.len() {
            if parts[i] == "offset" && i + 1 < parts.len() {
                offset = parts[i + 1].parse().unwrap_or(12);
            } else if parts[i] == "kind" && i + 1 < parts.len() {
                kind = parts[i + 1].to_string();
            }
        }

        let fetch_type = if kind == "all" { "newest" } else { &kind };
        let items = self.get_album_list(fetch_type, 24, offset).await?;
        let next_offset = offset + 24;
        let continuation = if items.len() >= 24 {
            Some(format!("offset:{next_offset}:kind:{kind}"))
        } else {
            None
        };

        let title = match kind.as_str() {
            "recent" => "More Recently Played",
            "frequent" => "More Most Played",
            "starred" => "More Favorites",
            "random" => "More Random Albums",
            _ => "More From Your Library",
        };

        let sections = if items.is_empty() {
            Vec::new()
        } else {
            vec![Section {
                title: title.to_string(),
                subtitle: None,
                items,
                kind: Some(kind),
            }]
        };

        Ok(HomePage {
            chips: Vec::new(),
            sections,
            continuation,
        })
    }

    pub async fn get_artists(&self) -> Result<Vec<BrowseItem>, Error> {
        let data: ArtistsContainer = self.get_json("getArtists", &[]).await?;
        let indexes = data.artists.map(|a| a.index).unwrap_or_default();
        let mut items = Vec::new();

        for idx in indexes {
            for art in idx.artist {
                let cover = art
                    .cover_art
                    .as_deref()
                    .or(art.artist_image_url.as_deref())
                    .map(|id| self.cover_art_url(id, Some(300)));
                items.push(artist_to_browse_item(&art, cover));
            }
        }

        Ok(items)
    }

    pub async fn get_artist(&self, id: &str) -> Result<ArtistPage, Error> {
        let p_id = [("id", id)];
        let p_top = [("artist", id), ("count", "10")];
        let (art_res, info_res, top_res) = tokio::join!(
            self.get_json::<ArtistContainer>("getArtist", &p_id),
            self.get_json::<ArtistInfo2Container>("getArtistInfo2", &p_id),
            self.get_json::<TopSongsContainer>("getTopSongs", &p_top),
        );

        let art_data = art_res?;
        let artist_with_albums = art_data.artist.ok_or(Error::MissingData)?;

        let info = info_res.ok().and_then(|i| i.artist_info);
        let bio = info.as_ref().and_then(|i| i.biography.clone());
        let similar = info.map(|i| i.similar_artist).unwrap_or_default();
        let top_songs = top_res.ok().and_then(|t| t.top_songs).map(|s| s.song).unwrap_or_default();

        Ok(artist_with_albums_to_page(
            &artist_with_albums,
            |cid| self.cover_art_url(cid, Some(600)),
            bio,
            top_songs,
            similar,
        ))
    }

    pub async fn get_album(&self, id: &str) -> Result<AlbumPage, Error> {
        let p_id = [("id", id)];
        let (album_res, info_res) = tokio::join!(
            self.get_json::<AlbumContainer>("getAlbum", &p_id),
            self.get_json::<AlbumInfoContainer>("getAlbumInfo2", &p_id),
        );

        let album_data = album_res?;
        let album_with_songs = album_data.album.ok_or(Error::MissingData)?;
        let notes = info_res.ok().and_then(|i| i.album_info).and_then(|i| i.notes);

        Ok(album_with_songs_to_page(
            &album_with_songs,
            |cid| self.cover_art_url(cid, Some(600)),
            notes,
        ))
    }

    pub async fn get_playlists(&self) -> Result<Vec<BrowseItem>, Error> {
        let data: PlaylistsContainer = self.get_json("getPlaylists", &[]).await?;
        let playlists = data.playlists.map(|p| p.playlist).unwrap_or_default();

        Ok(playlists
            .into_iter()
            .map(|p| {
                let cover = p.cover_art.as_deref().map(|cid| self.cover_art_url(cid, Some(400)));
                playlist_to_browse_item(&p, cover)
            })
            .collect())
    }

    pub async fn get_playlist(&self, id: &str) -> Result<PlaylistPage, Error> {
        let data: PlaylistContainer = self.get_json("getPlaylist", &[("id", id)]).await?;
        let playlist = data.playlist.ok_or(Error::MissingData)?;

        Ok(playlist_with_songs_to_page(
            &playlist,
            |cid| self.cover_art_url(cid, Some(600)),
            Some(&self.username),
        ))
    }

    pub async fn create_playlist(&self, name: &str, song_ids: &[String]) -> Result<String, Error> {
        let mut params = vec![("name", name)];
        let id_strs: Vec<(&str, &str)> = song_ids.iter().map(|id| ("songId", id.as_str())).collect();
        params.extend(id_strs);

        let data: PlaylistContainer = self.get_json("createPlaylist", &params).await?;
        let pl = data.playlist.ok_or(Error::MissingData)?;
        Ok(pl.playlist.id)
    }

    pub async fn update_playlist(
        &self,
        id: &str,
        name: Option<&str>,
        comment: Option<&str>,
        song_ids_to_add: &[String],
        song_indexes_to_remove: &[usize],
    ) -> Result<(), Error> {
        let mut params = vec![("playlistId", id)];
        if let Some(n) = name {
            params.push(("name", n));
        }
        if let Some(c) = comment {
            params.push(("comment", c));
        }
        let add_strs: Vec<(&str, &str)> = song_ids_to_add.iter().map(|s| ("songIdToAdd", s.as_str())).collect();
        params.extend(add_strs);

        let rem_strs: Vec<String> = song_indexes_to_remove.iter().map(|idx| idx.to_string()).collect();
        let rem_tuples: Vec<(&str, &str)> = rem_strs.iter().map(|s| ("songIndexToRemove", s.as_str())).collect();
        params.extend(rem_tuples);

        let _: serde_json::Value = self.get_json("updatePlaylist", &params).await?;
        Ok(())
    }

    pub async fn delete_playlist(&self, id: &str) -> Result<(), Error> {
        let _: serde_json::Value = self.get_json("deletePlaylist", &[("id", id)]).await?;
        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<SearchResults, Error> {
        let data: SearchResult3Container = self
            .get_json(
                "search3",
                &[
                    ("query", query),
                    ("artistCount", "10"),
                    ("albumCount", "20"),
                    ("songCount", "30"),
                ],
            )
            .await?;

        let res = data.search_result.unwrap_or(SearchResult3 {
            artist: Vec::new(),
            album: Vec::new(),
            song: Vec::new(),
        });

        let artists = res
            .artist
            .into_iter()
            .map(|a| {
                let cover = a.cover_art.as_deref().map(|c| self.cover_art_url(c, Some(300)));
                artist_to_browse_item(&a, cover)
            })
            .collect();

        let albums = res
            .album
            .into_iter()
            .map(|a| {
                let cover = self.cover_art_url(&a.cover_art.as_deref().unwrap_or(&a.id), Some(400));
                album_to_browse_item(&a, Some(cover))
            })
            .collect();

        let songs = res
            .song
            .into_iter()
            .map(|s| {
                let cover = s.cover_art.as_deref().map(|c| self.cover_art_url(c, Some(200)));
                child_to_song_item(&s, cover)
            })
            .collect();

        Ok(SearchResults {
            songs,
            albums,
            artists,
            playlists: Vec::new(),
        })
    }

    pub async fn get_song(&self, id: &str) -> Result<SongItem, Error> {
        let data: SongContainer = self.get_json("getSong", &[("id", id)]).await?;
        let song = data.song.ok_or(Error::MissingData)?;
        let cover = song.cover_art.as_deref().map(|c| self.cover_art_url(c, Some(400)));
        Ok(child_to_song_item(&song, cover))
    }

    pub async fn get_lyrics(&self, song_id: &str) -> Result<Option<Vec<TimedLyricLine>>, Error> {
        let data_res: Result<LyricsListContainer, _> = self.get_json("getLyricsBySongId", &[("id", song_id)]).await;
        if let Ok(data) = data_res {
            if let Some(list) = data.lyrics_list {
                for struct_lyric in list.structured_lyrics {
                    if struct_lyric.synced.unwrap_or(false) && !struct_lyric.line.is_empty() {
                        let lines = struct_lyric
                            .line
                            .into_iter()
                            .map(|l| TimedLyricLine {
                                text: l.value,
                                start_ms: l.start.unwrap_or(0),
                                end_ms: None,
                            })
                            .collect();
                        return Ok(Some(lines));
                    }
                }
            }
        }

        // Fallback to getLyrics endpoint
        let fallback_res: Result<SubsonicEnvelope<serde_json::Value>, _> = {
            let mut url = self.endpoint_url("getLyrics")?;
            url.query_pairs_mut().append_pair("id", song_id);
            let res = self.http.get(url).send().await?;
            res.json().await
        };

        if let Ok(env) = fallback_res {
            if let Some(val) = env.response.data {
                if let Some(lyrics_obj) = val.get("lyrics") {
                    if let Some(content) = lyrics_obj.get("content").and_then(|c| c.as_str()) {
                        let parsed = parse_lrc(content);
                        if !parsed.is_empty() {
                            return Ok(Some(parsed));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    pub async fn star(&self, id: &str) -> Result<(), Error> {
        let _: serde_json::Value = self.get_json("star", &[("id", id)]).await?;
        Ok(())
    }

    pub async fn unstar(&self, id: &str) -> Result<(), Error> {
        let _: serde_json::Value = self.get_json("unstar", &[("id", id)]).await?;
        Ok(())
    }

    pub async fn scrobble(&self, id: &str, submission: bool) -> Result<(), Error> {
        let sub_str = if submission { "true" } else { "false" };
        let _: serde_json::Value = self.get_json("scrobble", &[("id", id), ("submission", sub_str)]).await?;
        Ok(())
    }

    pub async fn get_similar_songs(&self, id: &str, count: u32) -> Result<Vec<SongItem>, Error> {
        let data: SimilarSongs2Container = self
            .get_json(
                "getSimilarSongs2",
                &[("id", id), ("count", &count.to_string())],
            )
            .await?;

        let songs = data.similar_songs.map(|s| s.song).unwrap_or_default();
        Ok(songs
            .into_iter()
            .map(|s| {
                let cover = s.cover_art.as_deref().map(|c| self.cover_art_url(c, Some(300)));
                child_to_song_item(&s, cover)
            })
            .collect())
    }

    pub async fn get_genres(&self) -> Result<Vec<Genre>, Error> {
        let data: GenresContainer = self.get_json("getGenres", &[]).await?;
        Ok(data.genres.map(|g| g.genre).unwrap_or_default())
    }

    pub async fn set_rating(&self, id: &str, rating: u32) -> Result<(), Error> {
        let _: serde_json::Value = self
            .get_json("setRating", &[("id", id), ("rating", &rating.to_string())])
            .await?;
        Ok(())
    }

    pub async fn add_to_playlist_single(&self, playlist_id: &str, song_id: &str) -> Result<(), Error> {
        self.update_playlist(playlist_id, None, None, &[song_id.to_string()], &[]).await
    }

    pub async fn remove_from_playlist_single(&self, playlist_id: &str, song_index: usize) -> Result<(), Error> {
        self.update_playlist(playlist_id, None, None, &[], &[song_index]).await
    }

    pub async fn start_scan(&self) -> Result<ScanStatus, Error> {
        let data: ScanStatusContainer = self.get_json("startScan", &[]).await?;
        data.scan_status.ok_or(Error::MissingData)
    }

    pub async fn get_scan_status(&self) -> Result<ScanStatus, Error> {
        let data: ScanStatusContainer = self.get_json("getScanStatus", &[]).await?;
        data.scan_status.ok_or(Error::MissingData)
    }

    pub async fn get_starred2(&self) -> Result<Starred2, Error> {
        let data: Starred2Container = self.get_json("getStarred2", &[]).await?;
        Ok(data.starred2.unwrap_or(Starred2 {
            artist: Vec::new(),
            album: Vec::new(),
            song: Vec::new(),
        }))
    }

    pub async fn get_random_songs(&self, size: u32) -> Result<Vec<SongItem>, Error> {
        let data: RandomSongsContainer = self
            .get_json("getRandomSongs", &[("size", &size.to_string())])
            .await?;
        let songs = data.random_songs.map(|s| s.song).unwrap_or_default();
        Ok(songs
            .into_iter()
            .map(|s| {
                let cover = s
                    .cover_art
                    .as_deref()
                    .or(s.album_id.as_deref())
                    .map(|c| self.cover_art_url(c, Some(300)));
                child_to_song_item(&s, cover)
            })
            .collect())
    }
}

fn generate_salt() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(123456789);
    format!("{nanos:x}")
}

fn compute_token(password: &str, salt: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

fn parse_lrc(text: &str) -> Vec<TimedLyricLine> {
    let mut lines = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('[') {
            continue;
        }
        if let Some(close_bracket) = trimmed.find(']') {
            let timestamp_str = &trimmed[1..close_bracket];
            let lyric_text = trimmed[close_bracket + 1..].trim().to_string();
            if let Some(ms) = parse_lrc_timestamp(timestamp_str) {
                lines.push(TimedLyricLine {
                    text: lyric_text,
                    start_ms: ms,
                    end_ms: None,
                });
            }
        }
    }
    lines
}

fn parse_lrc_timestamp(ts: &str) -> Option<i64> {
    let parts: Vec<&str> = ts.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let mins: f64 = parts[0].parse().ok()?;
    let secs: f64 = parts[1].parse().ok()?;
    Some(((mins * 60.0 + secs) * 1000.0) as i64)
}
