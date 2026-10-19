// src/ui/results_view.rs
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::game::session::GameSession;

pub struct ResultsView;

impl ResultsView {
    pub fn render(frame: &mut Frame, session: &GameSession, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(12),   // Stats & Rank
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header
        let is_fail = session.failed;
        let title_text = if is_fail { " ÉCHEC DU MORCEAU " } else { " MORCEAU TERMINÉ ! " };
        let title_color = if is_fail { Color::Red } else { Color::Green };

        let header_p = Paragraph::new(Line::from(vec![
            Span::styled(title_text, Style::default().fg(Color::Black).bg(title_color).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(session.map.pretty_title(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(header_p, chunks[0]);

        // Corps des résultats
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(45), // Grand Badge de Rang
                Constraint::Percentage(55), // Statistiques détaillées
            ])
            .split(chunks[1]);

        let rank = session.rank();
        let rank_color = match rank {
            "SS" => Color::Magenta,
            "S" => Color::Yellow,
            "A" => Color::Green,
            "B" => Color::Cyan,
            "C" => Color::LightYellow,
            _ => Color::Red,
        };

        // Dessin stylisé du rang
        let rank_lines = vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled("   ____    _    _   _ _  __ ", Style::default().fg(rank_color))),
            Line::from(Span::styled("  |  _ \\  / \\  | \\ | | |/ / ", Style::default().fg(rank_color))),
            Line::from(Span::styled("  | |_) |/ _ \\ |  \\| | ' /  ", Style::default().fg(rank_color))),
            Line::from(Span::styled("  |  _ </ ___ \\| |\\  | . \\  ", Style::default().fg(rank_color))),
            Line::from(Span::styled("  |_| \\_\\_/   \\_\\_| \\_|_|\\_\\ ", Style::default().fg(rank_color))),
            Line::from(""),
            Line::from(Span::styled(format!("       ★  {}  ★", rank), Style::default().fg(rank_color).add_modifier(Modifier::BOLD))),
        ];

        let rank_p = Paragraph::new(rank_lines)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" RANG ").border_style(Style::default().fg(rank_color)));
        frame.render_widget(rank_p, content_chunks[0]);

        // Détails des statistiques
        let stats_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(" SCORE FINAL : ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:07}", session.score), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" PRÉCISION (ACCURACY) : ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:.2}%", session.accuracy()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" MEILLEUR COMBO : ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{}", session.best_combo), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" NOTES FRAPPÉES (✓) : ", Style::default().fg(Color::Green)),
                Span::styled(format!("{}", session.hits), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled(" NOTES RATÉES (✗) : ", Style::default().fg(Color::Red)),
                Span::styled(format!("{}", session.misses), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" TOTAL NOTES MAP : ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{}", session.map.notes.len()), Style::default().fg(Color::Gray)),
            ]),
        ];

        let stats_p = Paragraph::new(stats_lines)
            .block(Block::default().borders(Borders::ALL).title(" STATISTIQUES ").border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(stats_p, content_chunks[1]);

        // Footer
        let footer_p = Paragraph::new(Line::from(vec![
            Span::styled(" [R] Rejouer ce morceau  •  [Entrée / ESC] Retour à la sélection ", Style::default().fg(Color::Gray)),
        ]))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(footer_p, chunks[2]);
    }
}
