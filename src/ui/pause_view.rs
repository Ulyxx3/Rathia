// src/ui/pause_view.rs
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub struct PauseView;

impl PauseView {
    pub fn render(frame: &mut Frame, area: Rect) {
        // Centrer un modal de 40x12
        let modal_w = 46.min(area.width);
        let modal_h = 10.min(area.height);

        let x = (area.width.saturating_sub(modal_w)) / 2;
        let y = (area.height.saturating_sub(modal_h)) / 2;
        let modal_area = Rect::new(x, y, modal_w, modal_h);

        frame.render_widget(Clear, modal_area);

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled("  PAUSE  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled(" [P / ESC / ESPACE] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("Reprendre"),
            ]),
            Line::from(vec![
                Span::styled(" [R] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("Recommencer"),
            ]),
            Line::from(vec![
                Span::styled(" [Q] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw("Quitter vers la sélection"),
            ]),
        ];

        let modal_p = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" PARTIE EN PAUSE ")
                    .border_style(Style::default().fg(Color::Yellow)),
            );

        frame.render_widget(modal_p, modal_area);
    }
}
