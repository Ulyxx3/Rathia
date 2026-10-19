// src/map/loader.rs
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use super::parser::parse_map;
use super::types::MapData;

#[derive(Debug, Clone)]
pub struct MapEntry {
    pub path: PathBuf,
    pub filename: String,
    pub title: String,
    pub artist: String,
    pub difficulty_name: String,
    pub length_ms: u32,
    pub note_count: usize,
    pub category: String, // ex: "Easy", "Hard", etc.
}

impl MapEntry {
    pub fn display_title(&self) -> String {
        if self.artist.is_empty() {
            self.title.clone()
        } else {
            format!("{} - {}", self.artist, self.title)
        }
    }
}

/// Scanner récursivement un dossier pour trouver toutes les maps (.sspm, .phxm, .txt)
pub fn scan_maps_directory(dir: &Path) -> Vec<MapEntry> {
    let mut entries = Vec::new();
    if !dir.exists() {
        return entries;
    }

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            if ext == "sspm" || ext == "phxm" || ext == "txt" {
                let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
                
                // Catégorie = sous-dossier direct dans maps/ s'il existe
                let category = if let Ok(rel) = path.strip_prefix(dir) {
                    if let Some(parent) = rel.parent() {
                        let comp = parent.components().next();
                        comp.map(|c| c.as_os_str().to_string_lossy().to_string()).unwrap_or_else(|| "General".to_string())
                    } else {
                        "General".to_string()
                    }
                } else {
                    "General".to_string()
                };

                let filename_lower = filename.to_lowercase();
                if filename_lower.contains("readme") || filename_lower.contains("archive") {
                    continue;
                }

                // Lire les métadonnées en lisant le fichier
                if let Ok(bytes) = std::fs::read(path) {
                    if let Ok(map) = parse_map(&bytes, &filename) {
                        if map.notes.is_empty() {
                            continue;
                        }
                        let difficulty_name = map.difficulty_label().to_string();
                        let note_count = map.notes.len();
                        let length_ms = map.length_ms;
                        let title = if map.title.is_empty() { "Sans titre".to_string() } else { map.title };
                        let artist = map.artist;
                        entries.push(MapEntry {
                            path: path.to_path_buf(),
                            filename,
                            title,
                            artist,
                            difficulty_name,
                            length_ms,
                            note_count,
                            category,
                        });
                    }
                }
            }
        }
    }

    // Trier par catégorie puis titre
    entries.sort_by(|a, b| {
        a.category.cmp(&b.category)
            .then_with(|| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
    });

    entries
}

/// Charger une map complète depuis un fichier
pub fn load_map_from_file(path: &Path) -> Result<MapData, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(path)?;
    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("map.sspm");
    let map = parse_map(&bytes, filename)?;
    Ok(map)
}

/// Charger uniquement la cover d'une map si disponible
pub fn load_map_cover(path: &Path) -> Option<Vec<u8>> {
    let bytes = std::fs::read(path).ok()?;
    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("map.sspm");
    let map = parse_map(&bytes, filename).ok()?;
    if !map.cover.is_empty() {
        Some(map.cover)
    } else {
        None
    }
}
