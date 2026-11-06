use subsonic::models::*;
use subsonic::ui_models::*;
use subsonic::adapter::*;

#[test]
fn test_duration_formatting() {
    assert_eq!(format_duration(Some(45)), Some("0:45".to_string()));
    assert_eq!(format_duration(Some(125)), Some("2:05".to_string()));
    assert_eq!(format_duration(Some(3665)), Some("1:01:05".to_string()));
    assert_eq!(format_duration(None), None);
}

#[test]
fn test_child_to_song_item() {
    let child = Child {
        id: "song_123".to_string(),
        parent: Some("album_456".to_string()),
        is_dir: Some(false),
        title: "Test Track".to_string(),
        album: Some("Test Album".to_string()),
        artist: Some("Test Artist".to_string()),
        track: Some(1),
        year: Some(2024),
        genre: Some("Rock".to_string()),
        cover_art: Some("cover_789".to_string()),
        size: Some(1024000),
        content_type: Some("audio/flac".to_string()),
        suffix: Some("flac".to_string()),
        duration: Some(215),
        bit_rate: Some(960),
        path: Some("Music/Test.flac".to_string()),
        play_count: Some(42),
        disc_number: Some(1),
        created: None,
        starred: Some("2024-01-01".to_string()),
        album_id: Some("album_456".to_string()),
        artist_id: Some("artist_789".to_string()),
        item_type: Some("music".to_string()),
        user_rating: Some(5),
    };

    let song_item = child_to_song_item(&child, Some("http://localhost/cover".to_string()));
    assert_eq!(song_item.video_id, "song_123");
    assert_eq!(song_item.title, "Test Track");
    assert_eq!(song_item.artists, "Test Artist");
    assert_eq!(song_item.artist_id, Some("artist_789".to_string()));
    assert_eq!(song_item.album, Some("Test Album".to_string()));
    assert_eq!(song_item.duration, Some("3:35".to_string()));
    assert_eq!(song_item.rating, Some(Rating::Like));
    assert_eq!(song_item.thumbnail, Some("http://localhost/cover".to_string()));
}

#[test]
fn test_subsonic_envelope_deserialization() {
    let json_data = r#"{
        "subsonic-response": {
            "status": "ok",
            "version": "1.16.1",
            "type": "navidrome",
            "serverVersion": "0.52.0",
            "openSubsonic": true
        }
    }"#;

    let envelope: SubsonicEnvelope<serde_json::Value> = serde_json::from_str(json_data).unwrap();
    assert_eq!(envelope.response.status, "ok");
    assert_eq!(envelope.response.version, "1.16.1");
    assert_eq!(envelope.response.server_type, Some("navidrome".to_string()));
}

#[test]
fn test_starred2_deserialization() {
    let json_data = r#"{
        "subsonic-response": {
            "status": "ok",
            "version": "1.16.1",
            "starred2": {
                "song": [
                    {
                        "id": "s1",
                        "title": "Starred Song 1",
                        "artist": "Artist A",
                        "duration": 180
                    }
                ],
                "album": [
                    {
                        "id": "al1",
                        "name": "Starred Album 1",
                        "artist": "Artist A"
                    }
                ],
                "artist": [
                    {
                        "id": "ar1",
                        "name": "Artist A"
                    }
                ]
            }
        }
    }"#;

    let envelope: SubsonicEnvelope<Starred2Container> = serde_json::from_str(json_data).unwrap();
    assert_eq!(envelope.response.status, "ok");
    let starred = envelope.response.data.unwrap().starred2.unwrap();
    assert_eq!(starred.song.len(), 1);
    assert_eq!(starred.album.len(), 1);
    assert_eq!(starred.artist.len(), 1);
    assert_eq!(starred.song[0].title, "Starred Song 1");
}

#[test]
fn test_stream_and_cover_url_generation() {
    let client = subsonic::SubsonicClient::new("http://example.com:4533", "alice", "secret").unwrap();
    let stream_url = client.stream_url("song123", Some(320));
    assert!(stream_url.contains("http://example.com:4533/rest/stream"));
    assert!(stream_url.contains("id=song123"));
    assert!(stream_url.contains("maxBitRate=320"));
    assert!(stream_url.contains("u=alice"));

    let cover_url = client.cover_art_url("cover456", Some(500));
    assert!(cover_url.contains("http://example.com:4533/rest/getCoverArt"));
    assert!(cover_url.contains("id=cover456"));
    assert!(cover_url.contains("size=500"));
}
