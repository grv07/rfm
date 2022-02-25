use super::common::style::{active_block_style, border_style, selected_dir_style, title_span};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use walkdir::WalkDir;

// Create a seprate builder for this.
pub struct DirBlock<'a> {
    pub dirs: Vec<String>,
    pub selected_index: usize,
    pub list: List<'a>,
    length: usize,
}

impl<'a> DirBlock<'a> {
    pub fn dirs(path: String) -> Vec<String> {
        let mut list_item = Vec::new();
        for entry in WalkDir::new(path)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
        {
            let curr_file = entry.path().to_string_lossy().into_owned();
            list_item.push(curr_file);
        }
        list_item
    }

    pub fn list_item(path: String) -> Vec<ListItem<'a>> {
        let mut list_item = Vec::new();
        for entry in Self::dirs(path) {
            list_item.push(ListItem::new(entry));
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
            .title(title_span("Dir"))
            .borders(Borders::ALL)
            .border_style(style);
        self.list = self.list.clone().block(block);
    }

    pub fn new(path: String) -> DirBlock<'a> {
        let block = Block::default()
            .title(title_span("Dir"))
            .borders(Borders::ALL)
            .border_style(border_style());
        let list = Self::list_item(path.clone());
        let length = list.len();
        let list = List::new(list)
            .block(block)
            .highlight_symbol("##")
            .highlight_style(selected_dir_style());

        Self {
            dirs: Self::dirs(path),
            selected_index: 0,
            list: list,
            length: length,
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
