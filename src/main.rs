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

struct DirTree<'a> {
    selected_index: usize,
    items: List<'a>,
    length: usize,  
}

impl<'a> DirTree<'a> {
    fn files_list(path: &str) -> Vec<ListItem> {
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

    fn current_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.selected_index));
        state
    }

    fn new(path: &'a str, block: Block<'a>) -> Self {
        let items = Self::files_list(path);
        let length = items.len();
        let list = List::new(items)
            .block(block)
            .highlight_symbol(">>")
            .highlight_style(selected_dir_style());
        Self {
            selected_index: 0,
            items: list,
            length: length, 
        }
    }

    fn up(&mut self) {
        if self.selected_index < self.length-1 {
            self.selected_index += 1
        };
    }

    fn down(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}

struct AppState<'a> {
    dir_tree: DirTree<'a>,
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
    let block = Block::default()
        .title(get_title_span("Dir"))
        .borders(Borders::ALL);

    let mut dir_tree = DirTree::new("/home/tyagig/rfm", block);

    loop {
        terminal.draw(|f| ui(f, &mut dir_tree))?;
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('n') => {dir_tree.up();},
                KeyCode::Char('p') => { dir_tree.down(); } ,
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, dir_tree: &mut DirTree) {
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

    let mut state = dir_tree.current_state();
    let tree_widget = dir_tree.items.clone();
    f.render_stateful_widget(tree_widget, chunks[0], &mut state);

    let block = Block::default()
        .title(get_title_span("Files"))
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
