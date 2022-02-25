use super::common::style::{active_block_style, border_style, selected_dir_style, title_span};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use walkdir::WalkDir;

// Create a seprate builder for this.
pub struct FilesBlock<'a> {
    files: Vec<String>,
    pub list: List<'a>,
    length: usize,
    selected_index: usize,
}

impl<'a> FilesBlock<'a> {
    pub fn files(path: String) -> Vec<String> {
        let mut list_item = Vec::new();
        for entry in WalkDir::new(path)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let curr_file = entry.path().to_string_lossy().into_owned();
            list_item.push(curr_file);
        }
        list_item
    }

    pub fn list_item(path: String) -> Vec<ListItem<'a>> {
        let mut list_item = Vec::new();
        for entry in FilesBlock::files(path) {
            let curr_file = entry;
            list_item.push(ListItem::new(curr_file));
        }
        list_item
    }

    pub fn current_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.selected_index));
        state
    }

    pub fn set_active(&mut self, is_active: bool) {
        let mut style = active_block_style();
        if !is_active {
            style = border_style();
        }
        let block = Block::default()
            .title(title_span("Files"))
            .borders(Borders::ALL)
            .border_style(style);
        self.list = self.list.clone().block(block);
    }

    pub fn new(selected_dir: String) -> FilesBlock<'a> {
        let block = Block::default()
            .title(title_span("Files"))
            .borders(Borders::ALL);
        let files = FilesBlock::list_item(selected_dir.clone());
        let len = files.len();
        let list = List::new(files)
            .block(block)
            .highlight_symbol(">>")
            .highlight_style(selected_dir_style());
        Self {
            files: FilesBlock::files(selected_dir),
            list: list,
            length: len,
            selected_index: 0,
        }
    }

    pub fn page_up(&mut self, size: usize) {
        if self.selected_index > size {
            self.selected_index -= size;
            return;
        }
        if self.selected_index > 0 {
            self.selected_index = 0;
            return;
        }
    }

    pub fn page_down(&mut self, size: usize) {
        if self.selected_index < self.length - 1 - size {
            self.selected_index += size;
            return;
        }
        if self.selected_index < self.length - 1 {
            self.selected_index = self.length - 1;
            return;
        }
    }

    pub fn up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1
        }
    }

    pub fn down(&mut self) {
        if self.selected_index < self.length - 1 {
            self.selected_index += 1;
        }
    }
}
