use crate::tui::component::{Action, Component, ComponentState, Focus, FocusState};
use anyhow::{Context, Result, bail};
use edtui::{
    EditorEventHandler, EditorState, EditorTheme, EditorView, LineNumbers, Lines, SyntaxHighlighter,
};
use log::{error, trace};
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Padding, Widget};
use std::path::PathBuf;
use syntect::parsing::SyntaxSet;

pub struct Editor {
    pub state: EditorState,
    pub event_handler: EditorEventHandler,
    pub file_path: Option<std::path::PathBuf>,
    syntax_set: SyntaxSet,
    pub(crate) component_state: ComponentState,
}

impl Editor {
    pub const ID: &str = "Editor";

    pub fn new(path: &std::path::Path) -> Self {
        trace!(target:Self::ID, "Building {}", Self::ID);
        Editor {
            state: EditorState::default(),
            event_handler: EditorEventHandler::default(),
            file_path: Some(path.to_owned()),
            syntax_set: SyntaxSet::load_defaults_nonewlines(),
            component_state: ComponentState::default().with_help_text(concat!(
                "CTRL+S: Save file | ALT+(←/h): Previous tab | ALT+(l/→): Next tab |",
                " All other input is handled by vim"
            )),
        }
    }

    pub fn reload_contents(&mut self) -> Result<()> {
        trace!(target:Self::ID, "Reloading editor file contents {:?}", self.file_path);
        match self.file_path.clone() {
            None => {
                error!(target:Self::ID, "Failed to reload editor contents with None file_path");
                bail!("Failed to reload editor contents with None file_path")
            }
            Some(path) => self.set_contents(&path),
        }
    }

    pub fn set_contents(&mut self, path: &std::path::Path) -> Result<()> {
        trace!(target:Self::ID, "Setting Editor contents from path {:?}", path);
        if let Ok(contents) = std::fs::read_to_string(path) {
            let lines: Vec<_> = contents
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect();
            self.file_path = Some(PathBuf::from(path));
            self.state.lines = Lines::new(lines);
            self.state.cursor.row = 0;
            self.state.cursor.col = 0;
        }
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = &self.file_path {
            trace!(target:Self::ID, "Saving Editor contents {:?}", path);
            return std::fs::write(path, self.state.lines.to_string()).map_err(|e| e.into());
        };
        error!(target:Self::ID, "Failed saving Editor contents; file_path was None");
        bail!("File not saved. No file path set.")
    }
}

impl Widget for &mut Editor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lang = self
            .file_path
            .as_ref()
            .and_then(|p| p.extension())
            .map(|e| e.to_str().unwrap_or("md"))
            .unwrap_or("md");
        let lang_name = self
            .syntax_set
            .find_syntax_by_extension(lang)
            .map(|s| s.name.to_string())
            .unwrap_or_else(|| String::from("Unknown"));

        EditorView::new(&mut self.state)
            .wrap(true)
            .theme(
                EditorTheme::default().block(
                    Block::default()
                        .title(lang_name.to_owned())
                        .title_style(Style::default().fg(Color::Yellow))
                        .title_alignment(Alignment::Right)
                        .borders(Borders::ALL)
                        .padding(Padding::new(0, 0, 0, 1))
                        .style(Style::default().fg(self.component_state.get_active_color())),
                ),
            )
            .syntax_highlighter(SyntaxHighlighter::new("dracula", lang).ok())
            .tab_width(2)
            .line_numbers(LineNumbers::Absolute)
            .render(area, buf);
    }
}

impl Component for Editor {
    fn handle_event(&mut self, event: Event) -> Result<Action> {
        if let Some(key_event) = event.as_key_event() {
            // Handle events here that should not be passed on to the vim emulation handler.
            match self.handle_key_events(key_event)? {
                Action::Handled => return Ok(Action::Handled),
                _ => {}
            }
        }
        self.event_handler.on_event(event, &mut self.state);
        Ok(Action::Pass)
    }

    /// The events for the vim emulation should be handled by EditorEventHandler::on_event.
    /// These events are custom to the clide application.
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action> {
        match key {
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.save().context("Failed to save file.")?;
                Ok(Action::Handled)
            }
            // For other events not handled here, pass to the vim emulation handler.
            _ => Ok(Action::Noop),
        }
    }

    fn is_active(&self) -> bool {
        self.component_state.focus == Focus::Active
    }
}
