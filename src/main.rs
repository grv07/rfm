use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
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

fn selected_dir_style() -> Style {
    Style::default()
        .fg(Color::LightBlue)
        .bg(Color::Black)
        .add_modifier(Modifier::ITALIC)
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
        .highlight_style(selected_dir_style())
}

struct DirTree {
    selected_index: usize,
    items: List,
}

impl DirTree {
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

    fn new(path: &str) -> Self {
        let list = List::new(Self::get_files_list(path))
            .block(block)
            .highlight_symbol(">>")
            .highlight_style(selected_dir_style());
        Self {
            selected_index: 0,
            items: list,
        }
    }

    fn up(&mut self) {
        self.selected_index += 1;
    }

    fn down() {
        self.selected_index -= 1;
    }
}

struct AppState {
    dir_tree: DirTree,
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run app
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err);
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
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
    let mut state = ListState::default();
    state.select(Some(2));
    f.render_stateful_widget(tree_widget(block), chunks[0], &mut state);
    let block = Block::default()
        .title(get_title_span("Files"))
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
