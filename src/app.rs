// src/app.rs
use std::path::Path;
use std::time::Instant;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::audio::AudioEngine;
use crate::game::session::{GameSession, GameSettings};
use crate::map::loader::{load_map_cover, load_map_from_file, scan_maps_directory, MapEntry};
use crate::ui::image_view::render_cover_halfblocks;
use crate::ui::{GameView, MenuView, PauseView, ResultsView, SettingsView, SongSelectView};
use ratatui::text::Line;

#[derive(Debug, PartialEq, Eq)]
pub enum AppScreen {
    Menu { selected: usize },
    SongSelect { selected: usize, scroll: usize },
    Playing,
    Paused,
    Results,
    Settings { selected: usize },
}

pub struct App {
    pub screen: AppScreen,
    pub maps: Vec<MapEntry>,
    pub session: Option<GameSession>,
    pub settings: GameSettings,
    pub audio: AudioEngine,
    pub running: bool,
    pub last_tick: Instant,
    pub terminal_size: (u16, u16),
    pub cover_cache: Option<(usize, Vec<Line<'static>>)>,
}

impl App {
    pub fn new() -> Self {
        let maps = scan_maps_directory(Path::new("maps"));
        Self {
            screen: AppScreen::Menu { selected: 0 },
            maps,
            session: None,
            settings: GameSettings::default(),
            audio: AudioEngine::new(),
            running: true,
            last_tick: Instant::now(),
            terminal_size: (100, 30),
            cover_cache: None,
        }
    }

    pub fn start_song(&mut self, index: usize) {
        if let Some(entry) = self.maps.get(index) {
            if let Ok(map_data) = load_map_from_file(&entry.path) {
                let session = GameSession::new(map_data, self.settings.clone());
                self.session = Some(session);
                self.screen = AppScreen::Playing;
            }
        }
    }

    pub fn restart_current_song(&mut self) {
        if let Some(ref current) = self.session {
            let map_clone = current.map.clone();
            self.audio.stop_song();
            self.session = Some(GameSession::new(map_clone, self.settings.clone()));
            self.screen = AppScreen::Playing;
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f64();
        self.last_tick = now;

        if let Some(ref mut session) = self.session {
            if self.screen == AppScreen::Playing {
                session.update(dt, &mut self.audio);

                if session.failed || session.completed {
                    self.screen = AppScreen::Results;
                }
            }
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match self.screen {
            AppScreen::Menu { ref mut selected } => match key.code {
                KeyCode::Up => {
                    if *selected > 0 { *selected -= 1; } else { *selected = 2; }
                }
                KeyCode::Down => {
                    if *selected < 2 { *selected += 1; } else { *selected = 0; }
                }
                KeyCode::Enter => match *selected {
                    0 => self.screen = AppScreen::SongSelect { selected: 0, scroll: 0 },
                    1 => self.screen = AppScreen::Settings { selected: 0 },
                    _ => self.running = false,
                },
                KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                _ => {}
            },

            AppScreen::SongSelect { ref mut selected, ref mut scroll } => match key.code {
                KeyCode::Up => {
                    if *selected > 0 {
                        *selected -= 1;
                        if *selected < *scroll {
                            *scroll = *selected;
                        }
                    }
                }
                KeyCode::Down => {
                    if !self.maps.is_empty() && *selected + 1 < self.maps.len() {
                        *selected += 1;
                        let visible_height = self.terminal_size.1.saturating_sub(6) as usize;
                        if *selected >= *scroll + visible_height {
                            *scroll += 1;
                        }
                    }
                }
                KeyCode::Enter => {
                    let sel = *selected;
                    self.start_song(sel);
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.screen = AppScreen::Menu { selected: 0 };
                }
                _ => {}
            },

            AppScreen::Playing => match key.code {
                KeyCode::Char('p') | KeyCode::Esc => {
                    if let Some(ref mut session) = self.session {
                        session.paused = true;
                    }
                    self.audio.pause_song();
                    self.screen = AppScreen::Paused;
                }
                KeyCode::Char('r') => {
                    self.restart_current_song();
                }
                KeyCode::Char('q') => {
                    self.audio.stop_song();
                    self.screen = AppScreen::SongSelect { selected: 0, scroll: 0 };
                }
                // Déplacement clavier optionnel
                KeyCode::Left => {
                    if let Some(ref mut session) = self.session {
                        session.cursor_x = (session.cursor_x - 0.15).max(-1.0);
                    }
                }
                KeyCode::Right => {
                    if let Some(ref mut session) = self.session {
                        session.cursor_x = (session.cursor_x + 0.15).min(1.0);
                    }
                }
                KeyCode::Up => {
                    if let Some(ref mut session) = self.session {
                        session.cursor_y = (session.cursor_y + 0.15).min(1.0);
                    }
                }
                KeyCode::Down => {
                    if let Some(ref mut session) = self.session {
                        session.cursor_y = (session.cursor_y - 0.15).max(-1.0);
                    }
                }
                _ => {}
            },

            AppScreen::Paused => match key.code {
                KeyCode::Char('p') | KeyCode::Esc | KeyCode::Char(' ') => {
                    if let Some(ref mut session) = self.session {
                        session.paused = false;
                    }
                    self.audio.resume_song();
                    self.screen = AppScreen::Playing;
                }
                KeyCode::Char('r') => {
                    self.restart_current_song();
                }
                KeyCode::Char('q') => {
                    self.audio.stop_song();
                    self.screen = AppScreen::SongSelect { selected: 0, scroll: 0 };
                }
                _ => {}
            },

            AppScreen::Results => match key.code {
                KeyCode::Char('r') => {
                    self.restart_current_song();
                }
                KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') => {
                    self.audio.stop_song();
                    self.screen = AppScreen::SongSelect { selected: 0, scroll: 0 };
                }
                _ => {}
            },

            AppScreen::Settings { ref mut selected } => match key.code {
                KeyCode::Up => {
                    if *selected > 0 { *selected -= 1; } else { *selected = 2; }
                }
                KeyCode::Down => {
                    if *selected < 2 { *selected += 1; } else { *selected = 0; }
                }
                KeyCode::Left => match *selected {
                    0 => self.settings.approach_rate = (self.settings.approach_rate - 2.0).max(10.0),
                    1 => self.settings.sensitivity = (self.settings.sensitivity - 0.1).max(0.2),
                    2 => self.settings.no_fail = !self.settings.no_fail,
                    _ => {}
                },
                KeyCode::Right => match *selected {
                    0 => self.settings.approach_rate = (self.settings.approach_rate + 2.0).min(70.0),
                    1 => self.settings.sensitivity = (self.settings.sensitivity + 0.1).min(3.0),
                    2 => self.settings.no_fail = !self.settings.no_fail,
                    _ => {}
                },
                KeyCode::Char(' ') => {
                    if *selected == 2 {
                        self.settings.no_fail = !self.settings.no_fail;
                    }
                }
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                    self.screen = AppScreen::Menu { selected: 1 };
                }
                _ => {}
            },
        }
    }

    pub fn handle_mouse_event(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::Moved | MouseEventKind::Drag(_) => {
                if self.screen == AppScreen::Playing || self.screen == AppScreen::Paused {
                    let (width, height) = self.terminal_size;

                    // Zone Canvas dans GameView :
                    // Gauche: 16 cols (HP/Combo), Droite: 18 cols (Stats), Haut: 3 cols (Header), Bas: 3 cols (Footer)
                    // Le Canvas a une bordure Borders::ALL (+1 pad de chaque côté)
                    let inner_x = 16.0 + 1.0;
                    let inner_w = (width.saturating_sub(16 + 18 + 2)).max(1) as f32;
                    let inner_y = 3.0 + 1.0;
                    let inner_h = (height.saturating_sub(3 + 3 + 2)).max(1) as f32;

                    let mx = mouse.column as f32;
                    let my = mouse.row as f32;

                    // Coordonnées relatives [0.0, 1.0] dans la zone intérieure de dessin du Canvas
                    let rel_x = (mx - inner_x) / inner_w;
                    let rel_y = (my - inner_y) / inner_h;

                    // Conversion exacte alignée avec ARENA_P_FACTOR du Canvas (0.85)
                    let p = crate::ui::game_view::ARENA_P_FACTOR as f32;
                    let norm_x = (((rel_x - 0.5) * 2.0) / p) * self.settings.sensitivity;
                    let norm_y = (((0.5 - rel_y) * 2.0) / p) * self.settings.sensitivity;

                    if let Some(ref mut session) = self.session {
                        session.set_cursor(norm_x, norm_y);
                    }
                }
            }
            MouseEventKind::ScrollDown => {
                if let AppScreen::SongSelect { ref mut selected, ref mut scroll } = self.screen {
                    if !self.maps.is_empty() && *selected + 1 < self.maps.len() {
                        *selected += 1;
                        let visible = self.terminal_size.1.saturating_sub(6) as usize;
                        if *selected >= *scroll + visible {
                            *scroll += 1;
                        }
                    }
                }
            }
            MouseEventKind::ScrollUp => {
                if let AppScreen::SongSelect { ref mut selected, ref mut scroll } = self.screen {
                    if *selected > 0 {
                        *selected -= 1;
                        if *selected < *scroll {
                            *scroll = *selected;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
        terminal.draw(|frame| {
            let area = frame.area();
            self.terminal_size = (area.width, area.height);

            match self.screen {
                AppScreen::Menu { selected } => {
                    MenuView::render(frame, selected, area);
                }
                AppScreen::SongSelect { selected, scroll } => {
                    // Mettre à jour le cache de cover si la map sélectionnée a changé
                    if self.cover_cache.as_ref().map(|(idx, _)| *idx) != Some(selected) {
                        let lines = if let Some(entry) = self.maps.get(selected) {
                            if let Some(cover_bytes) = load_map_cover(&entry.path) {
                                render_cover_halfblocks(&cover_bytes, 24, 12).unwrap_or_default()
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        };
                        self.cover_cache = Some((selected, lines));
                    }

                    let cover_slice = self.cover_cache.as_ref().map(|(_, l)| l.as_slice());
                    SongSelectView::render(frame, &self.maps, selected, scroll, cover_slice, area);
                }
                AppScreen::Playing => {
                    if let Some(ref session) = self.session {
                        GameView::render(frame, session, area);
                    }
                }
                AppScreen::Paused => {
                    if let Some(ref session) = self.session {
                        GameView::render(frame, session, area);
                        PauseView::render(frame, area);
                    }
                }
                AppScreen::Results => {
                    if let Some(ref session) = self.session {
                        ResultsView::render(frame, session, area);
                    }
                }
                AppScreen::Settings { selected } => {
                    SettingsView::render(frame, &self.settings, selected, area);
                }
            }
        })?;
        Ok(())
    }
}
