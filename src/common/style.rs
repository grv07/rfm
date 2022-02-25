use tui::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub fn title_style() -> Style {
    Style::default()
        .fg(Color::LightGreen)
        .bg(Color::Black)
        .add_modifier(Modifier::BOLD)
}

pub fn border_style() -> Style {
    Style::default().fg(Color::White)
}

pub fn active_block_style() -> Style {
    Style::default().fg(Color::Yellow)
}

pub fn selected_dir_style() -> Style {
    Style::default()
        .fg(Color::LightBlue)
        .bg(Color::Black)
        .add_modifier(Modifier::ITALIC)
}

pub fn title_span(text: &str) -> Span {
    Span::styled(text, title_style())
}
