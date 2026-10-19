// src/map/types.rs
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Fenêtre de frappe en millisecondes
pub const HIT_WINDOW_MS: f64 = 65.0;

/// Taille de la hitbox dans la grille normalisée [-1, 1]
pub const HIT_BOX_SIZE: f32 = 0.55;

/// Taille de la grille en unités monde
pub const GRID_SIZE: f32 = 3.0;

/// Noms des difficultés
pub const DIFFICULTIES: [&str; 6] = ["N/A", "Easy", "Medium", "Hard", "Insane", "Illogical"];

/// Une note de la map
#[derive(Debug, Clone, PartialEq)]
pub struct NoteData {
    pub index: usize,
    pub millisecond: u32,
    pub x: f32, // [-1.0, 1.0]
    pub y: f32, // [-1.0, 1.0]
}

/// Une map chargée et parsée
#[derive(Debug, Clone)]
pub struct MapData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub mappers: Vec<String>,
    pub difficulty: u8,
    pub difficulty_name: String,
    pub length_ms: u32,
    pub audio: Vec<u8>,
    pub cover: Vec<u8>,
    pub audio_ext: String,
    pub notes: Vec<NoteData>,
}

impl MapData {
    pub fn pretty_title(&self) -> String {
        if self.artist.is_empty() {
            self.title.clone()
        } else {
            format!("{} - {}", self.artist, self.title)
        }
    }

    pub fn pretty_mappers(&self) -> String {
        if self.mappers.is_empty() {
            "Unknown".to_string()
        } else {
            self.mappers.join(", ")
        }
    }

    pub fn difficulty_label(&self) -> &str {
        if !self.difficulty_name.is_empty() {
            &self.difficulty_name
        } else if (self.difficulty as usize) < DIFFICULTIES.len() {
            DIFFICULTIES[self.difficulty as usize]
        } else {
            "Custom"
        }
    }

    pub fn detect_audio_ext(bytes: &[u8]) -> &'static str {
        if bytes.len() >= 4 && &bytes[0..4] == b"OggS" {
            "ogg"
        } else {
            "mp3"
        }
    }
}

/// Métadonnées JSON du format PHXM
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PhxmMetadata {
    #[serde(rename = "ID", default)]
    pub id: String,
    #[serde(rename = "Title", default)]
    pub title: String,
    #[serde(rename = "Artist", default)]
    pub artist: String,
    #[serde(rename = "Mappers", default)]
    pub mappers: Vec<String>,
    #[serde(rename = "Difficulty", default)]
    pub difficulty: u8,
    #[serde(rename = "DifficultyName", default)]
    pub difficulty_name: String,
    #[serde(rename = "Length", default)]
    pub length: u32,
    #[serde(rename = "HasAudio", default)]
    pub has_audio: bool,
    #[serde(rename = "HasCover", default)]
    pub has_cover: bool,
    #[serde(rename = "HasVideo", default)]
    pub has_video: bool,
    #[serde(rename = "AudioExt", default)]
    pub audio_ext: String,
}
