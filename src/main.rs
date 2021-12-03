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

fn default_border_style() -> Style {
    Style::default().fg(Color::White)
}

fn active_border_style() -> Style {
    Style::default().fg(Color::Blue)
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
    list: List<'a>,
    length: usize,
    is_active: bool,
}

impl<'a> DirTree<'a> {
    fn files_list(path: &str) -> Vec<ListItem> {
        let mut list_item = Vec::new();
        for entry in WalkDir::new(path)
            .max_depth(10)
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

    fn check_active() {

    }

    fn new(path: &'a str, block: Block<'a>) -> Self {
        let list = Self::files_list(path);
        let length = list.len();
        let list = List::new(list)
            .block(block)
            .highlight_symbol(">>")
            .highlight_style(selected_dir_style());
        Self {
            selected_index: 0,
            list: list,
            length: length,
            is_active: false,
        }
    }

    fn page_up(&mut self, size: usize) {
        if self.selected_index > size {
            self.selected_index -= size;
            return;
        }
        if self.selected_index > 0 {
            self.selected_index = 0;
            return;
        }
    }

    fn page_down(&mut self, size: usize) {
        if self.selected_index < self.length - 1 - size {
            self.selected_index += size;
            return;
        }
        if self.selected_index < self.length - 1 {
            self.selected_index = self.length - 1;
            return;
        }
    }

    fn up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1
        }
    }

    fn down(&mut self) {
        if self.selected_index < self.length - 1 {
            self.selected_index += 1;
        }
    }
}

struct FilesBlock<'a> {
    list: List<'a>,
    length: usize,
    selected_index: usize,
    is_active: bool,
}

impl<'a> FilesBlock<'a> {
    fn files_list(path: &str) -> Vec<ListItem> {
        let mut list_item = Vec::new();
        for entry in WalkDir::new(path)
            .max_depth(10)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let curr_file = entry.path().to_string_lossy().into_owned();
            list_item.push(ListItem::new(curr_file));
        }
        list_item
    }

    fn new(selected_dir: &'a str, block: Block<'a>) -> Self {
        let files = FilesBlock::files_list(selected_dir);
        let len = files.len();
        let list = List::new(files)
            .block(block)
            .highlight_symbol(">>")
            .highlight_style(selected_dir_style());
        Self {
            list: list,
            length: len,
            selected_index: 0,
            is_active: false,
        }
    }

    fn up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1
        }
    }

    fn down(&mut self) {
        if self.selected_index < self.length - 1 {
            self.selected_index += 1;
        }
    }
}

enum ActiveBlock {
    Dir,
    Files,
}

struct AppState<'a> {
    dir_block: DirTree<'a>,
    files_block: FilesBlock<'a>,
    active_block: ActiveBlock,
    is_dirty: bool,
    // check in loop if dirty only then try to modify the
    // app state.. helps to minimize the load on loop..
}

impl<'a> AppState<'a> {
    // will return block style for block.. it calc
    // based on current active state.
    //fn block_style();

    fn change_active_state(&mut self) {
        match self.active_block {
            ActiveBlock::Dir => {
                self.active_block = ActiveBlock::Files;
                self.dir_block.is_active = false;
                self.files_block.is_active = true;
            }
            ActiveBlock::Files => {
                self.active_block = ActiveBlock::Dir;
                self.files_block.is_active = false;
                self.dir_block.is_active = true;
            }
        }
    }

    fn new(dir_block: DirTree<'a>, files_block: FilesBlock<'a>, active_block: ActiveBlock) -> Self {
        Self {
            dir_block: dir_block,
            files_block: files_block,
            active_block: ActiveBlock::Dir,
            is_dirty: false,
        }
    }

    //fn page_up(&mut self, size: usize) {
    //    if self.selected_index > size {
    //        self.selected_index -= size;
    //        return;
    //    }
    //    if self.selected_index > 0 {
    //        self.selected_index = 0;
    //        return;
    //    }
    //}

    //fn page_down(&mut self, size: usize) {
    //    if self.selected_index < self.length - 1 - size {
    //        self.selected_index += size;
    //        return;
    //    }
    //    if self.selected_index < self.length - 1 {
    //        self.selected_index = self.length - 1;
    //        return;
    //    }
    //}

    //fn up(&mut self) {
    //    if self.selected_index > 0 {
    //        self.selected_index -= 1
    //    }
    //}

    //fn down(&mut self) {
    //    if self.selected_index < self.length - 1 {
    //        self.selected_index += 1;
    //    }
    //}
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
    let dir_block = Block::default()
        .title(get_title_span("Dir"))
        .borders(Borders::ALL)
        .border_style(active_border_style());

    let files_block = Block::default()
        .title(get_title_span("Files"))
        .borders(Borders::ALL);

    let mut dir_block = DirTree::new("/home/tyagig/rfm", dir_block);
    let files_block = FilesBlock::new("/home/tyagig/rfm", files_block);

    let mut app_state = AppState::new(dir_block, files_block, ActiveBlock::Dir);
    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('n') => {
                    //dir_block.down();
                }
                KeyCode::Char('p') => {
                    //dir_block.up();
                }
                KeyCode::PageDown => {
                    //dir_block.page_down(50);
                }
                KeyCode::PageUp => {
                    //dir_block.page_up(50);
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
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

    //let mut state = app_state.current_state();
    let dir_widget = app_state.dir_block.list.clone();
    //f.render_stateful_widget(tree_widget, chunks[0], &mut state);
    f.render_widget(dir_widget, chunks[0]);

    let files_widget = app_state.files_block.list.clone();
    f.render_widget(files_widget, chunks[1]);
}
