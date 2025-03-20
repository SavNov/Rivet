//use image::{DynamicImage, GenericImageView};
use lofty::file::TaggedFileExt;
use lofty::prelude::*;
use lofty::probe::Probe;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
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
pub fn extract_metadata(path: &Path) -> Option<(String, String, String)> {
    if !path.is_file() {
        panic!("ERROR: Path is not a file!")
    }

    let tagged_file = Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
    };

    let title_binding = tag.title();
    let title = title_binding.as_deref().unwrap_or("Unknown Title");
    let artist_binding = tag.artist();
    let artist = artist_binding.as_deref().unwrap_or("Unknown Artist");
    let album_binding = tag.album();
    let album = album_binding.as_deref().unwrap_or("Unknown Album");
    //let year_binding = tag.year();
    //let year = year_binding.as_deref().unwrap_or("Unknown Year");

    // let properties = tagged_file.properties;
    // let duration = properties.duration();
    // let seconds = duration.as_seconds() % 60;
    // println!("Title: {}, Album: {}, Artist: {}", title, album, artist);
    Some((title.to_string(), artist.to_string(), album.to_string()))
}

/// Extract album artwork from the music file's metadata.
// pub fn extract_artwork(path: &Path) -> Option<DynamicImage> {
//     let tagged_file = Probe::open(path).ok()?.read().ok()?;
//     let tag = tagged_file.primary_tag()?;
// }

/// Convert a ReplayGain string to a floating-point multiplier.
pub fn replaygain_to_multiplier(gain: &str) -> Option<f32> {
    gain.parse::<f32>()
        .ok()
        .map(|gain_db| 10f32.powf(gain_db / 20.0))
}
