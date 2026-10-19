// src/ui/image_view.rs
use image::{imageops::FilterType, GenericImageView};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// Convertit un buffer d'image brut (PNG/JPEG) en lignes de demi-blocs Unicode TrueColor
pub fn render_cover_halfblocks(
    cover_bytes: &[u8],
    width_cells: u16,
    height_cells: u16,
) -> Option<Vec<Line<'static>>> {
    if width_cells == 0 || height_cells == 0 {
        return None;
    }

    let img = image::load_from_memory(cover_bytes).ok()?;
    let target_w = width_cells as u32;
    let target_h = (height_cells * 2) as u32;
    let resized = img.resize_exact(target_w, target_h, FilterType::Triangle);

    let mut lines = Vec::with_capacity(height_cells as usize);

    for row in 0..height_cells as u32 {
        let mut spans = Vec::with_capacity(width_cells as usize);
        for col in 0..width_cells as u32 {
            let top_pixel = resized.get_pixel(col, row * 2);
            let bottom_pixel = resized.get_pixel(col, row * 2 + 1);

            let top_color = Color::Rgb(top_pixel[0], top_pixel[1], top_pixel[2]);
            let bottom_color = Color::Rgb(bottom_pixel[0], bottom_pixel[1], bottom_pixel[2]);

            // '▀' : haut = foreground, bas = background
            spans.push(Span::styled(
                "▀",
                Style::default().fg(top_color).bg(bottom_color),
            ));
        }
        lines.push(Line::from(spans));
    }

    Some(lines)
}
