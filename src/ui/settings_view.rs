// src/ui/settings_view.rs
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::game::session::GameSettings;

pub struct SettingsView;

impl SettingsView {
    pub fn render(frame: &mut Frame, settings: &GameSettings, selected_field: usize, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(8),    // Options
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header
        let header_p = Paragraph::new(Line::from(vec![
            Span::styled(" PARAMÈTRES DE JEU ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(header_p, chunks[0]);

        // Champs de réglage
        let mut lines = Vec::new();

        // 0. Approach Rate (Vitesse des notes)
        let is_0 = selected_field == 0;
        lines.push(Line::from(vec![
            Span::styled(if is_0 { "▶ " } else { "  " }, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("Vitesse d'approche (Approach Rate) : ", if is_0 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::White) }),
            Span::styled(format!("{:.1}", settings.approach_rate), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("  [← / →]", Style::default().fg(Color::DarkGray)),
        ]));
        lines.push(Line::from(""));

        // 1. Sensibilité souris
        let is_1 = selected_field == 1;
        lines.push(Line::from(vec![
            Span::styled(if is_1 { "▶ " } else { "  " }, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("Sensibilité souris : ", if is_1 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::White) }),
            Span::styled(format!("{:.1}x", settings.sensitivity), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("  [← / →]", Style::default().fg(Color::DarkGray)),
        ]));
        lines.push(Line::from(""));

        // 2. Mod No Fail
        let is_2 = selected_field == 2;
        let nf_str = if settings.no_fail { "ACTIVÉ" } else { "DÉSACTIVÉ" };
        let nf_color = if settings.no_fail { Color::Green } else { Color::Red };
        lines.push(Line::from(vec![
            Span::styled(if is_2 { "▶ " } else { "  " }, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("Mode No-Fail (Ne jamais mourir) : ", if is_2 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::White) }),
            Span::styled(nf_str, Style::default().fg(nf_color).add_modifier(Modifier::BOLD)),
            Span::styled("  [Espace / Entrée]", Style::default().fg(Color::DarkGray)),
        ]));

        let options_p = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" OPTIONS ").border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(options_p, chunks[1]);

        // Footer
        let footer_p = Paragraph::new(Line::from(vec![
            Span::styled(" [↑/↓] Choisir  •  [←/→] Modifier  •  [ESC / Entrée] Retour ", Style::default().fg(Color::Gray)),
        ]))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(footer_p, chunks[2]);
    }
}
