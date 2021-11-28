use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;

use walkdir::WalkDir;

//TASKS:
// Keep all of the style in seprate mod.
// Create a mod for layout building only.

fn title_style() -> Style {
    Style::default()
        .fg(Color::LightGreen)
        .bg(Color::Black)
        .add_modifier(Modifier::BOLD)
}

fn get_title_span(text: &str) -> Span {
    Span::styled(text, title_style())
}

fn get_files_list(path: &str) -> Vec<ListItem> {
    let mut list_item = Vec::new();
    for entry in WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let curr_file = entry.path().to_string_lossy().into_owned();
        list_item.push(ListItem::new(curr_file));
    }
    list_item
}

fn tree_widget(block: Block) -> List {
    List::new(get_files_list("/home/tyagig/rfm"))
        .block(block)
        .highlight_symbol(">>")
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .horizontal_margin(5)
            .vertical_margin(5)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(50),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default()
            .title(get_title_span("Dir"))
            .borders(Borders::ALL);
        f.render_widget(tree_widget(block), chunks[0]);
        let block = Block::default()
            .title(get_title_span("Files"))
            .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;
    Ok(())
}
