// src/ui/menu_view.rs
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct MenuView;

impl MenuView {
    pub fn render(frame: &mut Frame, selected_index: usize, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // Logo
                Constraint::Min(6),     // Options
                Constraint::Length(3),  // Footer
            ])
            .split(area);

        // Logo ASCII Rathia
        let logo_lines = vec![
            Line::from(Span::styled("  ____       _   _     _       ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(" |  _ \\ __ _| |_| |__ (_) __ _ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(" | |_) / _` | __| '_ \\| |/ _` |", Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(" |  _ < (_| | |_| | | | | (_| |", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(" |_| \\_\\__,_|\\__|_| |_|_|\\__,_|", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled("   Rhythia for the Ratty Terminal   ", Style::default().fg(Color::Yellow))),
        ];

        let logo_p = Paragraph::new(logo_lines)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(logo_p, chunks[0]);

        // Menu items
        let options = ["JOUER", "PARAMÈTRES", "QUITTER"];
        let mut item_lines = Vec::new();

        for (i, opt) in options.iter().enumerate() {
            if i == selected_index {
                item_lines.push(Line::from(vec![
                    Span::styled("  ▶ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(format!(" {} ", opt), Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
                ]));
            } else {
                item_lines.push(Line::from(vec![
                    Span::raw("    "),
                    Span::styled(format!(" {} ", opt), Style::default().fg(Color::Gray)),
                ]));
            }
            item_lines.push(Line::from(""));
        }

        let menu_block = Block::default()
            .borders(Borders::ALL)
            .title(" MENU PRINCIPAL ")
            .border_style(Style::default().fg(Color::DarkGray));

        let items_p = Paragraph::new(item_lines)
            .alignment(Alignment::Center)
            .block(menu_block);
        frame.render_widget(items_p, chunks[1]);

        // Footer
        let footer_p = Paragraph::new(Line::from(vec![
            Span::styled(" [↑/↓] Naviguer  •  [Entrée] Valider  •  [Q] Quitter ", Style::default().fg(Color::DarkGray)),
        ]))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));

        frame.render_widget(footer_p, chunks[2]);
    }
}
