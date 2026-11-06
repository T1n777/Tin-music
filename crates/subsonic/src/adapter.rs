use crate::models::{AlbumID3, AlbumWithSongsID3, ArtistID3, ArtistWithAlbumsID3, Child, Playlist, PlaylistWithSongs};
use crate::ui_models::{AlbumPage, ArtistCarousel, ArtistPage, ArtistRun, BrowseItem, PlaylistPage, Rating, SongItem};

pub fn format_duration(seconds: Option<u32>) -> Option<String> {
    let s = seconds?;
    let mins = s / 60;
    let secs = s % 60;
    if mins >= 60 {
        let hours = mins / 60;
        let rem_mins = mins % 60;
        Some(format!("{hours}:{rem_mins:02}:{secs:02}"))
    } else {
        Some(format!("{mins}:{secs:02}"))
    }
}

pub fn child_to_song_item(c: &Child, cover_url: Option<String>) -> SongItem {
    let artist_str = c.artist.clone().unwrap_or_default();
    let artist_runs = if let Some(ref aid) = c.artist_id {
        vec![ArtistRun {
            text: artist_str.clone(),
            id: Some(aid.clone()),
        }]
    } else if !artist_str.is_empty() {
        vec![ArtistRun {
            text: artist_str.clone(),
            id: None,
        }]
    } else {
        Vec::new()
    };

    let rating = if c.starred.is_some() {
        Some(Rating::Like)
    } else {
        None
    };

    SongItem {
        video_id: c.id.clone(),
        title: c.title.clone(),
        artists: artist_str,
        artist_id: c.artist_id.clone(),
        artist_runs,
        album: c.album.clone(),
        album_id: c.album_id.clone(),
        duration: format_duration(c.duration),
        play_count: c.play_count.map(|p| p.to_string()),
        thumbnail: cover_url,
        set_video_id: None,
        added_by: None,
        added_by_avatars: Vec::new(),
        added_at: None,
        rating,
        library: None,
        explicit: false,
        is_video: false,
        is_upload: false,
        ..Default::default()
    }
}

pub fn album_to_browse_item(a: &AlbumID3, cover_url: Option<String>) -> BrowseItem {
    let mut subtitle_parts = Vec::new();
    if let Some(ref art) = a.artist {
        subtitle_parts.push(art.clone());
    }
    if let Some(year) = a.year {
        subtitle_parts.push(year.to_string());
    }
    let subtitle = if subtitle_parts.is_empty() {
        None
    } else {
        Some(subtitle_parts.join(" • "))
    };

    let artist_runs = if let Some(ref art) = a.artist {
        vec![ArtistRun {
            text: art.clone(),
            id: a.artist_id.clone(),
        }]
    } else {
        Vec::new()
    };

    BrowseItem {
        kind: "album".to_string(),
        id: a.id.clone(),
        title: a.name.clone(),
        subtitle,
        thumbnail: cover_url,
        duration: format_duration(a.duration),
        album_id: Some(a.id.clone()),
        artist_runs,
        play_count: a.play_count.map(|p| p.to_string()),
        is_video: false,
        is_upload: false,
        explicit: false,
    }
}

pub fn artist_to_browse_item(a: &ArtistID3, cover_url: Option<String>) -> BrowseItem {
    let subtitle = a.album_count.map(|c| {
        if c == 1 {
            "1 album".to_string()
        } else {
            format!("{c} albums")
        }
    });

    BrowseItem {
        kind: "artist".to_string(),
        id: a.id.clone(),
        title: a.name.clone(),
        subtitle,
        thumbnail: cover_url,
        duration: None,
        album_id: None,
        artist_runs: vec![ArtistRun {
            text: a.name.clone(),
            id: Some(a.id.clone()),
        }],
        play_count: None,
        is_video: false,
        is_upload: false,
        explicit: false,
    }
}

pub fn playlist_to_browse_item(p: &Playlist, cover_url: Option<String>) -> BrowseItem {
    let subtitle = p.song_count.map(|c| {
        if c == 1 {
            "1 song".to_string()
        } else {
            format!("{c} songs")
        }
    });

    BrowseItem {
        kind: "playlist".to_string(),
        id: p.id.clone(),
        title: p.name.clone(),
        subtitle,
        thumbnail: cover_url,
        duration: format_duration(p.duration),
        album_id: None,
        artist_runs: Vec::new(),
        play_count: None,
        is_video: false,
        is_upload: false,
        explicit: false,
    }
}

pub fn album_with_songs_to_page(
    a: &AlbumWithSongsID3,
    cover_url_fn: impl Fn(&str) -> String,
    notes: Option<String>,
) -> AlbumPage {
    let cover = cover_url_fn(&a.album.cover_art.as_deref().unwrap_or(&a.album.id));
    let items = a
        .song
        .iter()
        .map(|s| {
            let thumb = s
                .cover_art
                .as_deref()
                .or(a.album.cover_art.as_deref())
                .map(&cover_url_fn);
            child_to_song_item(s, thumb)
        })
        .collect();

    let mut subtitle = Vec::new();
    if let Some(ref art) = a.album.artist {
        subtitle.push(art.clone());
    }
    if let Some(year) = a.album.year {
        subtitle.push(year.to_string());
    }

    let second_subtitle = format!(
        "{} songs • {}",
        a.song.len(),
        format_duration(a.album.duration).unwrap_or_else(|| "0:00".to_string())
    );

    let artist_runs = if let Some(ref art) = a.album.artist {
        vec![ArtistRun {
            text: art.clone(),
            id: a.album.artist_id.clone(),
        }]
    } else {
        Vec::new()
    };

    AlbumPage {
        title: Some(a.album.name.clone()),
        artist: a.album.artist.clone(),
        artist_id: a.album.artist_id.clone(),
        artist_runs,
        artist_thumbnail: None,
        subtitle: if subtitle.is_empty() {
            None
        } else {
            Some(subtitle.join(" • "))
        },
        second_subtitle: Some(second_subtitle),
        description: notes,
        thumbnail: Some(cover),
        items,
        continuation: None,
        explicit: false,
        playlist_id: None,
        in_library: a.album.starred.is_some(),
        sections: Vec::new(),
    }
}

pub fn artist_with_albums_to_page(
    art: &ArtistWithAlbumsID3,
    cover_url_fn: impl Fn(&str) -> String,
    bio: Option<String>,
    top_songs: Vec<Child>,
    similar: Vec<ArtistID3>,
) -> ArtistPage {
    let banner_cover = art
        .artist
        .artist_image_url
        .clone()
        .or_else(|| art.artist.cover_art.as_deref().map(&cover_url_fn));

    let top_song_items = top_songs
        .iter()
        .map(|s| {
            let thumb = s.cover_art.as_deref().map(&cover_url_fn);
            child_to_song_item(s, thumb)
        })
        .collect();

    let album_items = art
        .album
        .iter()
        .map(|a| {
            let thumb = a.cover_art.as_deref().map(&cover_url_fn);
            album_to_browse_item(a, thumb)
        })
        .collect();

    let mut sections = vec![ArtistCarousel {
        title: "Albums".to_string(),
        items: album_items,
        more_browse_id: None,
        more_params: None,
    }];

    if !similar.is_empty() {
        let similar_items = similar
            .iter()
            .map(|s| {
                let thumb = s.cover_art.as_deref().map(&cover_url_fn);
                artist_to_browse_item(s, thumb)
            })
            .collect();

        sections.push(ArtistCarousel {
            title: "Similar Artists".to_string(),
            items: similar_items,
            more_browse_id: None,
            more_params: None,
        });
    }

    ArtistPage {
        name: Some(art.artist.name.clone()),
        thumbnail: banner_cover.clone(),
        avatar: banner_cover,
        description: bio,
        subscribers: None,
        monthly_listeners: None,
        channel_id: art.artist.id.clone(),
        subscribed: art.artist.starred.is_some(),
        radio_playlist_id: None,
        top_songs: top_song_items,
        top_songs_id: None,
        sections,
    }
}

pub fn playlist_with_songs_to_page(
    p: &PlaylistWithSongs,
    cover_url_fn: impl Fn(&str) -> String,
    current_username: Option<&str>,
) -> PlaylistPage {
    let cover = p.playlist.cover_art.as_deref().map(&cover_url_fn);
    let items = p
        .entry
        .iter()
        .map(|s| {
            let thumb = s.cover_art.as_deref().map(&cover_url_fn);
            child_to_song_item(s, thumb)
        })
        .collect();

    let owned = match (&p.playlist.owner, current_username) {
        (Some(o), Some(u)) => o.eq_ignore_ascii_case(u),
        _ => true,
    };

    let count = p.playlist.song_count.unwrap_or(p.entry.len() as u32);
    let dur = format_duration(p.playlist.duration).unwrap_or_else(|| "0:00".to_string());
    let subtitle = format!("{count} songs • {dur}");

    PlaylistPage {
        title: Some(p.playlist.name.clone()),
        subtitle: Some(subtitle),
        thumbnail: cover.clone(),
        description: p.playlist.comment.clone(),
        privacy: if p.playlist.public.unwrap_or(false) {
            Some("PUBLIC".to_string())
        } else {
            Some("PRIVATE".to_string())
        },
        cover,
        items,
        continuation: None,
        owned,
        collaborative: false,
        sort_menu: None,
        suggestions: None,
    }
}
