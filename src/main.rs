mod common;
mod dir_block;
mod file_block;

use crate::dir_block::DirBlock;
use crate::file_block::FilesBlock;
use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::List,
    Frame, Terminal,
};

enum ActiveBlock {
    Dir,
    Files,
}

struct AppState<'a> {
    dir_block: DirBlock<'a>,
    files_block: FilesBlock<'a>,
    active_block: ActiveBlock,
    // both of selected will track the current selected files and dir in future will help to open
    // the file :)
    selected_dir: String,
    selected_file: String,
}

impl<'a> AppState<'a> {
    // will return block style for block.. it calc
    // based on current active state.

    fn switch_block(&mut self) {
        match self.active_block {
            ActiveBlock::Files => {
                self.active_block = ActiveBlock::Dir;
            }
            ActiveBlock::Dir => {
                self.active_block = ActiveBlock::Files;
            }
        }
    }

    fn activate_block(&mut self) {
        match self.active_block {
            ActiveBlock::Dir => {
                self.files_block.set_active(false);
                self.dir_block.set_active(true);
            }
            ActiveBlock::Files => {
                self.files_block.set_active(true);
                self.dir_block.set_active(false);
            }
        }
    }

    fn get_dir_widget(&mut self) -> List {
        self.activate_block();
        self.dir_block.list.clone()
    }

    fn get_files_widget(&mut self) -> List {
        self.activate_block();
        self.files_block.list.clone()
    }

    fn new(dir_block: DirBlock<'a>, files_block: FilesBlock<'a>) -> AppState<'a> {
        Self {
            dir_block: dir_block,
            files_block: files_block,
            active_block: ActiveBlock::Dir,
            selected_dir: String::from(""),
            selected_file: String::from(""),
        }
    }

    fn on_dir_change(&mut self) {
        let dir = self.dir_block.dirs[self.dir_block.selected_index].clone();
        self.files_block = FilesBlock::new(dir);
    }

    fn page_up(&mut self, size: usize) {
        match self.active_block {
            ActiveBlock::Dir => {
                self.dir_block.page_up(size);
            }
            ActiveBlock::Files => {
                self.files_block.page_up(size);
            }
        }
    }

    fn page_down(&mut self, size: usize) {
        match self.active_block {
            ActiveBlock::Dir => {
                self.dir_block.page_down(size);
            }
            ActiveBlock::Files => {
                self.files_block.page_down(size);
            }
        }
    }

    fn up(&mut self) {
        match self.active_block {
            ActiveBlock::Dir => {
                self.dir_block.up();
                self.on_dir_change();
            }
            ActiveBlock::Files => {
                self.files_block.up();
            }
        }
    }

    fn down(&mut self) {
        match self.active_block {
            ActiveBlock::Dir => {
                self.dir_block.down();
                self.on_dir_change();
            }
            ActiveBlock::Files => {
                self.files_block.down();
            }
        }
    }
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
    // extra performance issue
    let dirs = DirBlock::dirs(".".to_string());
    // don't create File block and Dir Block also in here create it through FileBlockBuilder and inside AppState new
    // only.
    let path = dirs.first();
    let dir_block = DirBlock::new(".".to_string());
    let files_block = FilesBlock::new(path.unwrap().to_string());

    let mut app_state = AppState::new(dir_block, files_block);
    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('n') => {
                    app_state.down();
                }
                KeyCode::Char('p') => {
                    app_state.up();
                }
                KeyCode::PageDown => {
                    app_state.page_down(50);
                }
                KeyCode::PageUp => {
                    app_state.page_up(50);
                }
                KeyCode::Left => {
                    app_state.switch_block();
                }
                KeyCode::Right => {
                    app_state.switch_block();
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

    let mut dir_state = app_state.dir_block.current_state();
    let dir_widget = app_state.get_dir_widget();
    f.render_stateful_widget(dir_widget, chunks[0], &mut dir_state);

    let mut files_state = app_state.files_block.current_state();
    let files_widget = app_state.get_files_widget();
    f.render_stateful_widget(files_widget, chunks[1], &mut files_state);
}
