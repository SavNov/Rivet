use lofty::{Probe, TaggedFileExt};
use walkdir::WalkDir;
use image::{DynamicImage, GenericImageView};
use std::path::{Path, PathBuf};

/// Scan the music directory and find supported music files (mp3, flac, etc.).
pub fn scan_music_files(music_dir: &str) -> Vec<PathBuf> {
    WalkDir::new(music_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            matches!(
                e.path().extension().and_then(|e| e.to_str()),
                Some("mp3") | Some("flac") | Some("ogg") | Some("wav") | Some("m4a") | Some("opus")
            )
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Extract metadata (e.g., title, artist) from a music file.
pub fn extract_metadata(path: &Path) -> Option<(String, String)> {
    let tagged_file = Probe::open(path).ok()?.read(true).ok()?;
    let tag = tagged_file.primary_tag()?;

    let title = tag.get_string(&lofty::ItemKey::TrackTitle).unwrap_or("Unknown").to_string();
    let artist = tag.get_string(&lofty::ItemKey::Artist).unwrap_or("Unknown").to_string();

    Some((title, artist))
}

/// Extract album artwork from the music file's metadata.
pub fn extract_artwork(path: &Path) -> Option<DynamicImage> {
    let tagged_file = Probe::open(path).ok()?.read(true).ok()?;
    let tag = tagged_file.primary_tag()?;

    if let Some(picture) = tag.pictures().first() {
        image::load_from_memory(&picture.data).ok()
    } else {
        None
    }
}

/// Convert a ReplayGain string to a floating-point multiplier.
pub fn replaygain_to_multiplier(gain: &str) -> Option<f32> {
    gain.parse::<f32>().ok().map(|gain_db| 10f32.powf(gain_db / 20.0))
}
