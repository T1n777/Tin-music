# Tin-music Feature Ideas & Future Roadmap

This document tracks feature requests, architecture proposals, and ideas to explore once the core client experience is fully polished.

---

## 1. Remote Server Music Uploads (Direct Navidrome Ingestion)

### Context & Motivation
Navidrome is a read-only indexer of an existing directory of audio files on the server. Neither Navidrome nor the OpenSubsonic/Subsonic API provides a native HTTP endpoint for uploading new tracks or albums from a client application.

Currently, adding music requires manual out-of-band transfers (e.g. CLI SCP/rsync, Docker volume mapping, or web file managers) followed by a library scan.

### Proposed Solutions

#### Approach A: Integrated SFTP / SCP Transfer (Recommended)
- **Settings Configuration**:
  - Server SSH/SFTP Host (e.g. `tinapple-server` or Tailscale IP)
  - SSH Port (default `22`)
  - Username & Authentication (SSH private key or password)
  - Remote Music Library Path (e.g. `/var/music`, `/home/tin/music`)
- **Workflow**:
  1. User clicks an **"Upload to Server"** button in Library / Settings or drops audio files/folders directly into the app.
  2. Tin-music reads track ID3 metadata (Artist, Album, Title) locally.
  3. Tin-music transfers the files via SFTP to the remote directory (e.g. creating `Music/Artist/Album/Track.ext`).
  4. Once transfer completes, Tin-music automatically triggers Navidrome's `startScan` API (`subsonic_client.start_scan()`).
  5. Navidrome indexes the new files and they immediately appear in the library without restarting the server.

#### Approach B: Local / Network Mount Integration (SMB / NFS / SSHFS)
- For setups where the server's music folder is mounted locally (e.g. at `/mnt/music` or `/media/server-music`):
  - Configure the local mount directory in Tin-music Settings.
  - App performs a direct file copy into the mount and calls Navidrome's `startScan`.

#### Approach C: WebDAV / Companion Upload Service
- Support pushing files via HTTP to an optional companion service running alongside Navidrome (such as a WebDAV server or FileBrowser instance).

---

## 2. Extended Audio & Transcoding Controls

- Transcoding bitrate selector in Settings (e.g. Max 128 kbps, 320 kbps, or Direct Stream / Uncompressed FLAC).
- ReplayGain / Loudness normalization presets tailored for desktop playback.
- Offline track / album caching to a local directory for listening while disconnected from the server.

---

## 3. Multi-Server Profiles

- Quick profile switcher in the top bar to switch between different Navidrome / Subsonic instances (e.g., Home Server, Cloud VPS, Friend's server).
