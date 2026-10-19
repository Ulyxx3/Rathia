// src/ui/game_view.rs
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CLine, Rectangle},
        Block, Borders, Gauge, Paragraph,
    },
    Frame,
};
use crate::game::session::GameSession;

const NOTE_COLORS: &[Color] = &[
    Color::Cyan,
    Color::Magenta,
    Color::LightGreen,
    Color::Yellow,
    Color::LightBlue,
    Color::LightRed,
];

pub struct GameView;

pub const ARENA_P_FACTOR: f64 = 0.85;
pub const HALF_W: f64 = 100.0;
pub const HALF_H: f64 = 75.0;

impl GameView {
    pub fn render(frame: &mut Frame, session: &GameSession, area: Rect) {
        // Layout principal : Header (3 lignes), Zone de jeu centrale, Footer HUD (3 lignes)
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(12),
                Constraint::Length(3),
            ])
            .split(area);

        Self::render_header(frame, session, main_chunks[0]);
        Self::render_arena(frame, session, main_chunks[1]);
        Self::render_footer(frame, session, main_chunks[2]);
    }

    fn render_header(frame: &mut Frame, session: &GameSession, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        // Titre et Difficulté
        let diff_color = match session.map.difficulty {
            1 => Color::Green,
            2 => Color::Yellow,
            3 => Color::LightRed,
            4 => Color::Red,
            5 => Color::Magenta,
            _ => Color::Cyan,
        };

        let title_spans = vec![
            Span::styled(" ♫ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(session.map.pretty_title(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(format!("[{}]", session.map.difficulty_label()), Style::default().fg(diff_color).add_modifier(Modifier::BOLD)),
        ];

        let title_p = Paragraph::new(Line::from(title_spans))
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(title_p, chunks[0]);

        // Progression de la chanson
        let total_secs = session.map.length_ms as f64 / 1000.0;
        let current_secs = (session.progress_ms / 1000.0).max(0.0).min(total_secs);
        let progress_ratio = if total_secs > 0.0 { (current_secs / total_secs).clamp(0.0, 1.0) } else { 0.0 };

        let time_label = format!(
            "{:02}:{:02} / {:02}:{:02}",
            (current_secs as u32) / 60,
            (current_secs as u32) % 60,
            (total_secs as u32) / 60,
            (total_secs as u32) % 60,
        );

        let progress_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)))
            .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .ratio(progress_ratio)
            .label(time_label);

        frame.render_widget(progress_gauge, chunks[1]);
    }

    fn render_arena(frame: &mut Frame, session: &GameSession, area: Rect) {
        // Découpage : Sidebar Gauche (Santé & Combo), Canvas 3D au Centre, Sidebar Droite (Score & Stats)
        let arena_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(16),
                Constraint::Min(20),
                Constraint::Length(18),
            ])
            .split(area);

        Self::render_left_sidebar(frame, session, arena_chunks[0]);
        Self::render_3d_canvas(frame, session, arena_chunks[1]);
        Self::render_right_sidebar(frame, session, arena_chunks[2]);
    }

    fn render_left_sidebar(frame: &mut Frame, session: &GameSession, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Min(4),
            ])
            .split(area);

        // Barre de vie (Health)
        let health_ratio = (session.health / 100.0).clamp(0.0, 1.0);
        let health_color = if session.health > 50.0 {
            Color::Green
        } else if session.health > 25.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        let health_block = Block::default()
            .borders(Borders::ALL)
            .title(" HP ")
            .border_style(Style::default().fg(health_color));

        let hp_gauge = Gauge::default()
            .block(health_block)
            .gauge_style(Style::default().fg(health_color).bg(Color::Black))
            .ratio(health_ratio)
            .label(format!("{:.0}%", session.health));

        frame.render_widget(hp_gauge, chunks[0]);

        // Combo
        let combo_color = if session.combo >= 100 {
            Color::Magenta
        } else if session.combo >= 50 {
            Color::Yellow
        } else if session.combo >= 20 {
            Color::Cyan
        } else {
            Color::White
        };

        let combo_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(format!(" {}", session.combo), Style::default().fg(combo_color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(" COMBO", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw(" Max: "),
                Span::styled(format!("{}", session.best_combo), Style::default().fg(Color::Gray)),
            ]),
        ];

        let combo_p = Paragraph::new(combo_lines)
            .block(Block::default().borders(Borders::ALL).title(" STREAK ").border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(combo_p, chunks[1]);
    }

    fn render_right_sidebar(frame: &mut Frame, session: &GameSession, area: Rect) {
        let rank = session.rank();
        let rank_color = match rank {
            "SS" => Color::Magenta,
            "S" => Color::Yellow,
            "A" => Color::Green,
            "B" => Color::Cyan,
            "C" => Color::LightYellow,
            _ => Color::Red,
        };

        let lines = vec![
            Line::from(vec![
                Span::styled(" SCORE", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::styled(format!(" {:07}", session.score), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" ACCURACY", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::styled(format!(" {:.2}%", session.accuracy()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw(" RANK: "),
                Span::styled(format!(" {} ", rank), Style::default().fg(rank_color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" ✓ Hit: ", Style::default().fg(Color::Green)),
                Span::raw(format!("{}", session.hits)),
            ]),
            Line::from(vec![
                Span::styled(" ✗ Miss: ", Style::default().fg(Color::Red)),
                Span::raw(format!("{}", session.misses)),
            ]),
        ];

        let stats_p = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" STATS ").border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(stats_p, area);
    }

    fn render_3d_canvas(frame: &mut Frame, session: &GameSession, area: Rect) {
        let half_w = HALF_W;
        let half_h = HALF_H;
        let bound_x = HALF_W / ARENA_P_FACTOR;
        let bound_y = HALF_H / ARENA_P_FACTOR;

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Rathia • Tunnel 3D (Hover with Mouse) ")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .marker(Marker::Braille)
            .x_bounds([-bound_x, bound_x])
            .y_bounds([-bound_y, bound_y])
            .paint(|ctx| {
                // 1. Dessiner le cadre de frappe au plan Z=0 (devant)
                let frame_color = Color::Rgb(40, 120, 220);
                // Bordure extérieure
                ctx.draw(&CLine { x1: -half_w, y1: -half_h, x2: half_w, y2: -half_h, color: frame_color });
                ctx.draw(&CLine { x1: half_w, y1: -half_h, x2: half_w, y2: half_h, color: frame_color });
                ctx.draw(&CLine { x1: half_w, y1: half_h, x2: -half_w, y2: half_h, color: frame_color });
                ctx.draw(&CLine { x1: -half_w, y1: half_h, x2: -half_w, y2: -half_h, color: frame_color });

                // 2. Le fond du tunnel (Z = profondeur max)
                let max_depth = session.settings.approach_distance as f64;
                let back_scale = 1.0 / (1.0 + max_depth * 0.08);
                let back_w = half_w * back_scale;
                let back_h = half_h * back_scale;
                let tunnel_edge_color = Color::Rgb(30, 45, 75);

                // Cadre de fond
                ctx.draw(&CLine { x1: -back_w, y1: -back_h, x2: back_w, y2: -back_h, color: tunnel_edge_color });
                ctx.draw(&CLine { x1: back_w, y1: -back_h, x2: back_w, y2: back_h, color: tunnel_edge_color });
                ctx.draw(&CLine { x1: back_w, y1: back_h, x2: -back_w, y2: back_h, color: tunnel_edge_color });
                ctx.draw(&CLine { x1: -back_w, y1: back_h, x2: -back_w, y2: -back_h, color: tunnel_edge_color });

                // 4 lignes de fuite reliant l'avant au fond
                ctx.draw(&CLine { x1: -half_w, y1: -half_h, x2: -back_w, y2: -back_h, color: tunnel_edge_color });
                ctx.draw(&CLine { x1: half_w, y1: -half_h, x2: back_w, y2: -back_h, color: tunnel_edge_color });
                ctx.draw(&CLine { x1: half_w, y1: half_h, x2: back_w, y2: back_h, color: tunnel_edge_color });
                ctx.draw(&CLine { x1: -half_w, y1: half_h, x2: -back_w, y2: back_h, color: tunnel_edge_color });

                // Anneau intermédiaire pour accentuer l'effet de profondeur
                let mid_scale = 1.0 / (1.0 + (max_depth * 0.4) * 0.08);
                let mid_w = half_w * mid_scale;
                let mid_h = half_h * mid_scale;
                let ring_color = Color::Rgb(25, 35, 60);
                ctx.draw(&CLine { x1: -mid_w, y1: -mid_h, x2: mid_w, y2: -mid_h, color: ring_color });
                ctx.draw(&CLine { x1: mid_w, y1: -mid_h, x2: mid_w, y2: mid_h, color: ring_color });
                ctx.draw(&CLine { x1: mid_w, y1: mid_h, x2: -mid_w, y2: mid_h, color: ring_color });
                ctx.draw(&CLine { x1: -mid_w, y1: mid_h, x2: -mid_w, y2: -mid_h, color: ring_color });

                // 3. Dessiner les notes qui s'approchent (du fond vers le joueur)
                let at = session.approach_time_secs() as f64;
                let ad = session.settings.approach_distance as f64;
                let progress = session.progress_ms;

                // Afficher le compte à rebours si pregame
                if session.progress_ms < 0.0 {
                    let countdown = (-session.progress_ms / 1000.0).ceil() as u32;
                    let cd_size = 15.0;
                    ctx.draw(&Rectangle {
                        x: -cd_size,
                        y: -cd_size,
                        width: cd_size * 2.0,
                        height: cd_size * 2.0,
                        color: Color::Yellow,
                    });
                    ctx.print(0.0, 0.0, format!(" GET READY: {} ", countdown));
                }

                // Filtrer et trier les notes visibles par profondeur décroissante (peintre)
                let mut visible_notes = Vec::new();
                for note in session.notes.iter() {
                    if note.is_hit || note.is_missed {
                        continue;
                    }
                    let time_diff = note.millisecond as f64 - progress;
                    let depth = (time_diff / (1000.0 * at)) * ad;

                    if depth >= -2.0 && depth <= ad {
                        visible_notes.push((note, depth));
                    }
                }
                // Tri du plus loin au plus proche pour rendu correct
                visible_notes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                for (note, depth) in visible_notes {
                    let scale = (1.0 / (1.0 + depth.max(0.0) * 0.08)) as f64;
                    // Coordonnées projetées en 3D
                    let proj_x = (note.x as f64) * half_w * scale;
                    let proj_y = (note.y as f64) * half_h * scale;

                    // Taille des notes (bien visibles, proportionnelles à la grille)
                    let base_size = 52.0;
                    let note_w = (base_size * scale).max(5.0);
                    let note_h = (base_size * scale * 0.68).max(3.5); // Ratio d'aspect pour terminal

                    let color = NOTE_COLORS[note.index % NOTE_COLORS.len()];

                    // Si la note est dans la hit window (< 55ms) : éclat lumineux
                    let is_in_hit_window = (progress - note.millisecond as f64).abs() <= 55.0;
                    let draw_color = if is_in_hit_window { Color::White } else { color };

                    // Dessin de la note (boîte rectangle néon en Braille)
                    ctx.draw(&Rectangle {
                        x: proj_x - note_w / 2.0,
                        y: proj_y - note_h / 2.0,
                        width: note_w,
                        height: note_h,
                        color: draw_color,
                    });

                    // Contour intérieur néon pour un rendu plein et éclatant
                    if depth < 12.0 {
                        let inner_w = note_w * 0.7;
                        let inner_h = note_h * 0.7;
                        ctx.draw(&Rectangle {
                            x: proj_x - inner_w / 2.0,
                            y: proj_y - inner_h / 2.0,
                            width: inner_w,
                            height: inner_h,
                            color,
                        });
                    }
                }

                // 4. Dessiner le curseur joueur (Carré blanc de la taille du curseur original)
                let cur_screen_x = session.cursor_x as f64 * half_w;
                let cur_screen_y = session.cursor_y as f64 * half_h;
                let cur_w = 14.0;
                let cur_h = 14.0 * 0.68;

                ctx.draw(&Rectangle {
                    x: cur_screen_x - cur_w / 2.0,
                    y: cur_screen_y - cur_h / 2.0,
                    width: cur_w,
                    height: cur_h,
                    color: Color::White,
                });
            });

        frame.render_widget(canvas, area);
    }

    fn render_footer(frame: &mut Frame, session: &GameSession, area: Rect) {
        let help_text = if session.paused {
            " [P / ESC] Reprendre  •  [R] Recommencer  •  [Q] Menu "
        } else {
            " [Souris] Déplacer le curseur  •  [P / ESC] Pause  •  [R] Restart  •  [Q] Quitter "
        };

        let footer_p = Paragraph::new(Line::from(vec![
            Span::styled(" RATHIA ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(help_text, Style::default().fg(Color::Gray)),
        ]))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));

        frame.render_widget(footer_p, area);
    }
}
