// src/ui/song_select_view.rs
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::map::loader::MapEntry;

pub struct SongSelectView;

impl SongSelectView {
    pub fn render(
        frame: &mut Frame,
        maps: &[MapEntry],
        selected_index: usize,
        scroll_offset: usize,
        cover_lines: Option<&[Line<'static>]>,
        area: Rect,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Content (List + Preview)
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header
        let header_p = Paragraph::new(Line::from(vec![
            Span::styled(" SÉLECTION DES MAPS ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(format!("  {} maps trouvées dans maps/", maps.len())),
        ]))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(header_p, chunks[0]);

        // Content : Liste à gauche (55%), Preview à droite (45%)
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(55),
                Constraint::Percentage(45),
            ])
            .split(chunks[1]);

        // Liste scrollable
        let max_visible = content_chunks[0].height.saturating_sub(2) as usize;
        let visible_maps = maps.iter().skip(scroll_offset).take(max_visible);

        let list_items: Vec<ListItem> = visible_maps
            .enumerate()
            .map(|(rel_idx, entry)| {
                let abs_idx = scroll_offset + rel_idx;
                let is_selected = abs_idx == selected_index;

                let diff_color = match entry.category.as_str() {
                    "Easy" => Color::Green,
                    "Medium" => Color::Yellow,
                    "Hard" => Color::LightRed,
                    "Logic" => Color::Magenta,
                    _ => Color::Cyan,
                };

                let prefix = if is_selected { "▶ " } else { "  " };

                let content = Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("[{}] ", entry.category), Style::default().fg(diff_color)),
                    Span::styled(
                        entry.display_title(),
                        if is_selected {
                            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        },
                    ),
                ]);

                ListItem::new(content)
            })
            .collect();

        let list_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Morceaux ({}/{}) ", if maps.is_empty() { 0 } else { selected_index + 1 }, maps.len()))
            .border_style(Style::default().fg(Color::Cyan));

        let list = List::new(list_items).block(list_block);
        frame.render_widget(list, content_chunks[0]);

        // Preview panneau de droite
        if let Some(entry) = maps.get(selected_index) {
            let duration_secs = entry.length_ms / 1000;
            let preview_lines = vec![
                Line::from(vec![
                    Span::styled(" TITRE: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&entry.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled(" ARTISTE: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(if entry.artist.is_empty() { "Inconnu" } else { &entry.artist }, Style::default().fg(Color::LightBlue)),
                ]),
                Line::from(vec![
                    Span::styled(" DIFFICULTÉ: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&entry.difficulty_name, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled(" CATÉGORIE: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&entry.category, Style::default().fg(Color::Magenta)),
                ]),
                Line::from(vec![
                    Span::styled(" DURÉE: ", Style::default().fg(Color::DarkGray)),
                    Span::raw(format!("{:02}:{:02}", duration_secs / 60, duration_secs % 60)),
                    Span::styled("  •  NOTES: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{}", entry.note_count), Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::styled(" FICHIER: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&entry.filename, Style::default().fg(Color::DarkGray)),
                ]),
            ];

            let has_cover = cover_lines.map(|c| !c.is_empty()).unwrap_or(false);

            if has_cover {
                let right_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(14), // Image de Cover (12 lignes de pixels + 2 bordures)
                        Constraint::Min(6),     // Détails texte
                    ])
                    .split(content_chunks[1]);

                if let Some(lines) = cover_lines {
                    let cover_p = Paragraph::new(lines.to_vec())
                        .alignment(ratatui::layout::Alignment::Center)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(" JAQUETTE DE LA MAP ")
                                .border_style(Style::default().fg(Color::Cyan)),
                        );
                    frame.render_widget(cover_p, right_chunks[0]);
                }

                let preview_p = Paragraph::new(preview_lines)
                    .block(Block::default().borders(Borders::ALL).title(" DÉTAILS ").border_style(Style::default().fg(Color::DarkGray)));
                frame.render_widget(preview_p, right_chunks[1]);
            } else {
                let preview_p = Paragraph::new(preview_lines)
                    .block(Block::default().borders(Borders::ALL).title(" DÉTAILS DE LA MAP ").border_style(Style::default().fg(Color::DarkGray)));
                frame.render_widget(preview_p, content_chunks[1]);
            }
        } else {
            let empty_p = Paragraph::new("Aucune map sélectionnée")
                .block(Block::default().borders(Borders::ALL).title(" DÉTAILS ").border_style(Style::default().fg(Color::DarkGray)));
            frame.render_widget(empty_p, content_chunks[1]);
        }

        // Footer
        let footer_p = Paragraph::new(Line::from(vec![
            Span::styled(" [↑/↓ / Molette] Naviguer  •  [Entrée] Jouer cette map  •  [ESC] Menu principal ", Style::default().fg(Color::Gray)),
        ]))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(footer_p, chunks[2]);
    }
}
