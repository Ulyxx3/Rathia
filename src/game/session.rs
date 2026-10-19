// src/game/session.rs
use crate::map::types::{MapData, HIT_WINDOW_MS, HIT_BOX_SIZE};
use crate::audio::AudioEngine;

#[derive(Debug, Clone)]
pub struct NotePlayState {
    pub index: usize,
    pub millisecond: u32,
    pub x: f32, // [-1.0, 1.0]
    pub y: f32, // [-1.0, 1.0]
    pub is_hit: bool,
    pub is_missed: bool,
}

#[derive(Debug, Clone)]
pub struct GameSettings {
    pub approach_rate: f32,
    pub approach_distance: f32,
    pub sensitivity: f32,
    pub no_fail: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            approach_rate: 35.0,
            approach_distance: 30.0,
            sensitivity: 1.0,
            no_fail: false,
        }
    }
}

pub struct GameSession {
    pub map: MapData,
    pub notes: Vec<NotePlayState>,
    pub progress_ms: f64,
    pub pregame_ms: f64,
    pub audio_started: bool,
    pub cursor_x: f32, // [-1.0, 1.0]
    pub cursor_y: f32, // [-1.0, 1.0]
    pub score: i64,
    pub combo: u32,
    pub best_combo: u32,
    pub hits: u32,
    pub misses: u32,
    pub health: f64,
    pub health_step: f64,
    pub failed: bool,
    pub completed: bool,
    pub paused: bool,
    pub settings: GameSettings,
    pub last_hit_delta_ms: Option<f64>,
}

impl GameSession {
    pub fn new(map: MapData, settings: GameSettings) -> Self {
        let notes = map.notes.iter().map(|n| NotePlayState {
            index: n.index,
            millisecond: n.millisecond,
            x: n.x,
            y: n.y,
            is_hit: false,
            is_missed: false,
        }).collect();

        Self {
            map,
            notes,
            progress_ms: -2500.0,
            pregame_ms: 2500.0,
            audio_started: false,
            cursor_x: 0.0,
            cursor_y: 0.0,
            score: 0,
            combo: 0,
            best_combo: 0,
            hits: 0,
            misses: 0,
            health: 100.0,
            health_step: 15.0,
            failed: false,
            completed: false,
            paused: false,
            settings,
            last_hit_delta_ms: None,
        }
    }

    pub fn accuracy(&self) -> f64 {
        let total = (self.hits + self.misses) as f64;
        if total == 0.0 { 100.0 } else { (self.hits as f64 / total) * 100.0 }
    }

    pub fn rank(&self) -> &'static str {
        if self.failed { return "F"; }
        let acc = self.accuracy();
        if acc >= 100.0 && self.misses == 0 { "SS" }
        else if acc >= 95.0 { "S" }
        else if acc >= 90.0 { "A" }
        else if acc >= 80.0 { "B" }
        else if acc >= 70.0 { "C" }
        else { "D" }
    }

    pub fn approach_time_secs(&self) -> f32 {
        self.settings.approach_distance / self.settings.approach_rate
    }

    /// Mettre à jour la session avec le delta temps et la synchronisation audio
    pub fn update(&mut self, dt_secs: f64, audio: &mut AudioEngine) {
        if self.paused || self.failed || self.completed {
            return;
        }

        // Compte à rebours initial avant le lancement de la musique
        if !self.audio_started {
            self.pregame_ms = (self.pregame_ms - dt_secs * 1000.0).max(0.0);
            self.progress_ms = -self.pregame_ms;

            if self.pregame_ms <= 0.0 {
                if !self.map.audio.is_empty() {
                    let _ = audio.play_song(&self.map.audio);
                }
                self.audio_started = true;
                self.progress_ms = 0.0;
            }
        } else {
            // Synchronisation avec l'horloge Kira haute fidélité
            if audio.is_playing() {
                self.progress_ms = audio.get_position_ms();
            } else {
                self.progress_ms += dt_secs * 1000.0;
            }
        }

        // Vérification de fin de chanson
        let total_length = self.map.length_ms as f64;
        let last_note_ms = self.notes.last().map(|n| n.millisecond as f64).unwrap_or(0.0);
        let end_threshold = total_length.max(last_note_ms + 1500.0);
        if self.progress_ms > end_threshold {
            self.completed = true;
            return;
        }

        // Détection Auto-Hit (hover-to-hit) et Misses
        let progress = self.progress_ms;
        let hit_box = HIT_BOX_SIZE;

        for note in self.notes.iter_mut() {
            if note.is_hit || note.is_missed {
                continue;
            }

            let delta = progress - note.millisecond as f64;

            // Trop loin dans le futur
            if delta < -HIT_WINDOW_MS {
                break; // Les notes sont triées chronologiquement
            }

            // Dans la hit window temporelle [-55ms, +55ms]
            if delta.abs() <= HIT_WINDOW_MS {
                let dx = (self.cursor_x - note.x).abs();
                let dy = (self.cursor_y - note.y).abs();

                if dx <= hit_box && dy <= hit_box {
                    // HIT !
                    note.is_hit = true;
                    self.hits += 1;
                    self.combo += 1;
                    self.best_combo = self.best_combo.max(self.combo);
                    self.score += 100 + (self.combo as i64 * 10);
                    self.last_hit_delta_ms = Some(delta);

                    // Health formula
                    self.health_step = (self.health_step / 1.45).max(15.0);
                    self.health = (self.health + self.health_step / 1.75).min(100.0);

                    audio.play_hit();
                    continue;
                }
            }

            // Dépassé la hit window (+55ms dans le passé) -> MISS !
            if delta > HIT_WINDOW_MS {
                note.is_missed = true;
                self.misses += 1;
                self.combo = 0;

                // Health decrease & escalation
                if !self.settings.no_fail {
                    self.health = (self.health - self.health_step).max(0.0);
                    self.health_step = (self.health_step * 1.2).min(100.0);

                    if self.health <= 0.0 {
                        self.failed = true;
                        audio.stop_song();
                        audio.play_fail();
                        return;
                    }
                }
            }
        }
    }

    /// Déplacer le curseur directement vers des coordonnées normalisées [-1.0, 1.0]
    pub fn set_cursor(&mut self, x: f32, y: f32) {
        self.cursor_x = x.clamp(-1.0, 1.0);
        self.cursor_y = y.clamp(-1.0, 1.0);
    }
}
