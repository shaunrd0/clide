use crate::tui::component::{Action, Component, Focus, FocusState};
use crate::tui::editor::Editor;
use anyhow::{Context, Result, anyhow};
use log::{error, info, trace, warn};
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Padding, Tabs, Widget};
use std::collections::HashMap;

// Render the tabs with keys as titles
// Tab keys can be file names.
// Render the editor using the key as a reference for lookup
pub struct EditorTab {
    pub(crate) editors: HashMap<String, Editor>,
    tab_order: Vec<String>,
    current_editor: usize,
}

impl EditorTab {
    pub const ID: &str = "EditorTab";

    pub fn new() -> Self {
        trace!(target:Self::ID, "Building {}", Self::ID);
        Self {
            editors: HashMap::new(),
            tab_order: Vec::new(),
            current_editor: 0,
        }
    }

    pub fn next_editor(&mut self) {
        let next = (self.current_editor + 1) % self.tab_order.len();
        trace!(target:Self::ID, "Moving from {} to next editor tab at {}", self.current_editor, next);
        self.set_tab_focus(Focus::Active, next);
        self.current_editor = next;
    }

    pub fn prev_editor(&mut self) {
        let prev = self
            .current_editor
            .checked_sub(1)
            .unwrap_or(self.tab_order.len() - 1);
        trace!(target:Self::ID, "Moving from {} to previous editor tab at {}", self.current_editor, prev);
        self.set_tab_focus(Focus::Active, prev);
        self.current_editor = prev;
    }

    pub fn get_editor_key(&self, index: usize) -> Option<String> {
        match self.tab_order.get(index) {
            None => {
                if !self.tab_order.is_empty() {
                    error!(target:Self::ID, "Failed to get editor tab key with invalid index {index}");
                }
                None
            }
            Some(key) => Some(key.to_owned()),
        }
    }

    pub fn current_editor(&self) -> Option<&Editor> {
        self.editors.get(&self.get_editor_key(self.current_editor)?)
    }

    pub fn current_editor_mut(&mut self) -> Option<&mut Editor> {
        self.editors
            .get_mut(&self.get_editor_key(self.current_editor)?)
    }

    pub fn set_current_tab_focus(&mut self, focus: Focus) {
        trace!(target:Self::ID, "Setting current tab {} focus to {:?}", self.current_editor, focus);
        self.set_tab_focus(focus, self.current_editor)
    }

    pub fn set_tab_focus(&mut self, focus: Focus, index: usize) {
        trace!(target:Self::ID, "Setting tab {} focus to {:?}", index, focus);
        if focus == Focus::Active && index != self.current_editor {
            // If we are setting another tab to active, disable the current one.
            trace!(
                target:Self::ID,
                "New tab {} focus set to Active; Setting current tab {} to Inactive",
                index,
                self.current_editor
            );
            self.set_current_tab_focus(Focus::Inactive);
        }
        match self.get_editor_key(index) {
            None => {
                error!(target:Self::ID, "Failed setting tab focus for invalid key {index}");
            }
            Some(key) => match self.editors.get_mut(&key) {
                None => {
                    error!(
                        target:Self::ID,
                        "Failed to update tab focus at index {} with invalid key: {}",
                        self.current_editor,
                        self.tab_order[self.current_editor]
                    )
                }
                Some(editor) => editor.component_state.set_focus(focus),
            },
        }
    }

    pub fn open_tab(&mut self, path: &std::path::Path) -> Result<()> {
        trace!(target:Self::ID, "Opening new EditorTab with path {:?}", path);
        if self
            .editors
            .contains_key(&path.to_string_lossy().to_string())
        {
            warn!(target:Self::ID, "EditorTab already opened with this file");
            return Ok(());
        }

        let path_str = path.to_string_lossy().to_string();
        self.tab_order.push(path_str.clone());
        let mut editor = Editor::new(path);
        editor.set_contents(path).context("Failed to open tab")?;
        self.editors.insert(path_str, editor);
        self.current_editor = self.tab_order.len() - 1;
        Ok(())
    }

    pub fn close_current_tab(&mut self) -> Result<()> {
        self.close_tab(self.current_editor)
    }

    pub fn close_tab(&mut self, index: usize) -> Result<()> {
        let key = self
            .tab_order
            .get(index)
            .ok_or_else(|| anyhow!("Failed to get tab order with invalid index {index}"))?
            .to_owned();
        match self.editors.remove(&key) {
            None => {
                error!(target:Self::ID, "Failed to remove editor tab {key} with invalid index {index}")
            }
            Some(_) => {
                self.prev_editor();
                self.tab_order.remove(index);
                info!(target:Self::ID, "Closed editor tab {key} at index {index}")
            }
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.editors.is_empty()
    }

    pub fn render(&mut self, tabs_area: Rect, editor_area: Rect, buf: &mut Buffer) {
        // TODO: Only file name is displayed in tab title, so files with the same name in different
        //    directories will appear confusing.
        let tab_titles = self.tab_order.iter().map(|t| {
            std::path::Path::new(t)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| String::from("Unknown"))
        });
        // Don't set border color based on ComponentState::focus, the Editor renders the border.
        Tabs::new(tab_titles)
            .select(self.current_editor)
            .divider("|")
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .padding(Padding::new(0, 0, 0, 0)),
            )
            .highlight_style(Style::default().fg(Color::LightRed))
            .render(tabs_area, buf);
        Widget::render(self, editor_area, buf);
    }
}

impl Widget for &mut EditorTab {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if let Some(editor) = self.current_editor_mut() {
            editor.render(area, buf);
        }
    }
}

impl Component for EditorTab {
    fn handle_event(&mut self, event: Event) -> Result<Action> {
        if let Some(key) = event.as_key_event() {
            let action = self.handle_key_events(key)?;
            match action {
                Action::Quit | Action::Handled => return Ok(action),
                _ => {}
            }
        }
        if let Some(editor) = self.current_editor_mut() {
            return editor.handle_event(event);
        }
        Ok(Action::Noop)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action> {
        match key {
            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::ALT,
                ..
            }
            | KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::ALT,
                ..
            } => {
                self.prev_editor();
                Ok(Action::Handled)
            }
            KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: KeyModifiers::ALT,
                ..
            }
            | KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::ALT,
                ..
            } => {
                self.next_editor();
                Ok(Action::Handled)
            }
            _ => Ok(Action::Noop),
        }
    }
}
