// src/audio/mod.rs
// ==============================================================================
// Rathia — Moteur Audio (Kira)
// Fournit une horloge haute précision (~0.02ms) synchronisée sur le thread audio
// ==============================================================================

use std::io::Cursor;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend,
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
    Tween,
};

static HIT_SFX_BYTES: &[u8] = include_bytes!("../../assets/sounds/hit.mp3");
static FAIL_SFX_BYTES: &[u8] = include_bytes!("../../assets/sounds/fail.mp3");

pub struct AudioEngine {
    manager: Option<AudioManager<DefaultBackend>>,
    song_handle: Option<StaticSoundHandle>,
    hit_sound: Option<StaticSoundData>,
    fail_sound: Option<StaticSoundData>,
    volume_master: f32,
    volume_music: f32,
    volume_sfx: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).ok();
        
        let hit_sound = StaticSoundData::from_cursor(Cursor::new(HIT_SFX_BYTES)).ok();
        let fail_sound = StaticSoundData::from_cursor(Cursor::new(FAIL_SFX_BYTES)).ok();

        Self {
            manager,
            song_handle: None,
            hit_sound,
            fail_sound,
            volume_master: 1.0,
            volume_music: 1.0,
            volume_sfx: 0.8,
        }
    }

    pub fn play_song(&mut self, audio_bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mut handle) = self.song_handle.take() {
            let _ = handle.stop(Tween::default());
        }

        if let Some(ref mut mgr) = self.manager {
            let cursor = Cursor::new(audio_bytes.to_vec());
            let sound_data = StaticSoundData::from_cursor(cursor)?;
            let vol = self.volume_master * self.volume_music;
            let settings = StaticSoundSettings::default().volume(vol);
            let handle = mgr.play(sound_data.with_settings(settings))?;
            self.song_handle = Some(handle);
        }
        Ok(())
    }

    pub fn stop_song(&mut self) {
        if let Some(mut handle) = self.song_handle.take() {
            let _ = handle.stop(Tween::default());
        }
    }

    pub fn pause_song(&mut self) {
        if let Some(ref mut handle) = self.song_handle {
            let _ = handle.pause(Tween::default());
        }
    }

    pub fn resume_song(&mut self) {
        if let Some(ref mut handle) = self.song_handle {
            let _ = handle.resume(Tween::default());
        }
    }

    pub fn is_playing(&self) -> bool {
        self.song_handle.as_ref().map(|h| h.state() == kira::sound::PlaybackState::Playing).unwrap_or(false)
    }

    /// Position exacte en millisecondes depuis le thread audio Kira
    pub fn get_position_ms(&self) -> f64 {
        if let Some(ref handle) = self.song_handle {
            handle.position() * 1000.0
        } else {
            0.0
        }
    }

    pub fn play_hit(&mut self) {
        if let (Some(ref mut mgr), Some(ref sfx)) = (&mut self.manager, &self.hit_sound) {
            let vol = self.volume_master * self.volume_sfx;
            let settings = StaticSoundSettings::default().volume(vol);
            let _ = mgr.play(sfx.clone().with_settings(settings));
        }
    }

    pub fn play_fail(&mut self) {
        if let (Some(ref mut mgr), Some(ref sfx)) = (&mut self.manager, &self.fail_sound) {
            let vol = self.volume_master * self.volume_sfx;
            let settings = StaticSoundSettings::default().volume(vol);
            let _ = mgr.play(sfx.clone().with_settings(settings));
        }
    }
}
