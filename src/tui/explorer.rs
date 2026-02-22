// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

use crate::tui::component::{Action, Component, ComponentState, Focus, FocusState};
use anyhow::{Context, Result, bail};
use libclide::fs::entry_meta::EntryMeta;
use log::trace;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use ratatui::layout::{Alignment, Position, Rect};
use ratatui::prelude::Style;
use ratatui::style::{Color, Modifier};
use ratatui::widgets::{Block, Borders, StatefulWidget, Widget};
use std::fs;
use std::path::{Path, PathBuf};
use tui_tree_widget::{Tree, TreeItem, TreeState};

#[derive(Debug)]
pub struct Explorer<'a> {
    root_path: EntryMeta,
    tree_items: TreeItem<'a, String>,
    tree_state: TreeState<String>,
    pub(crate) component_state: ComponentState,
}

impl<'a> Explorer<'a> {
    pub const ID: &'static str = "Explorer";

    pub fn new(path: &PathBuf) -> Result<Self> {
        trace!(target:Self::ID, "Building {}", Self::ID);
        let explorer = Explorer {
            root_path: EntryMeta::new(path)?,
            tree_items: Self::build_tree_from_path(path)?,
            tree_state: TreeState::default(),
            component_state: ComponentState::default().with_help_text(concat!(
                "(↑/k)/(↓/j): Select item | ←/h: Close folder | →/l: Open folder |",
                " Space: Open / close folder | Enter: Open file in new editor tab"
            )),
        };
        Ok(explorer)
    }

    /// Builds the file tree from a path using recursion.
    /// The identifiers used for the TreeItems are normalized. Symlinks are not resolved.
    /// Resolving symlinks would cause collisions on the TreeItem unique identifiers within the set.
    fn build_tree_from_path<P: AsRef<Path>>(p: P) -> Result<TreeItem<'static, String>> {
        let path = p.as_ref();
        let mut children = vec![];
        let path_meta = EntryMeta::new(path)?;
        if let Ok(entries) = fs::read_dir(&path_meta.abs_path) {
            let mut paths = entries
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()
                .context(format!(
                    "Failed to build vector of paths under directory: {:?}",
                    &path_meta.abs_path
                ))?;
            paths.sort();
            for entry_path in paths {
                let entry_meta = EntryMeta::new(&entry_path)?;
                if entry_meta.is_dir {
                    children.push(Self::build_tree_from_path(&entry_meta.abs_path)?);
                } else {
                    children.push(TreeItem::new_leaf(
                        entry_meta.abs_path.clone(),
                        entry_meta.file_name.clone(),
                    ));
                }
            }
        }

        // Note: The first argument is a unique identifier, where no 2 TreeItems may share the same.
        // For a file tree this is fine because we shouldn't list the same object twice.
        TreeItem::new(
            path_meta.abs_path.clone(),
            path_meta.file_name.clone(),
            children,
        )
        .context(format!(
            "Failed to build tree from path: {:?}",
            path_meta.abs_path
        ))
    }

    pub fn selected(&self) -> Result<String> {
        if let Some(path) = self.tree_state.selected().last() {
            return Ok(std::path::absolute(path)?
                .to_str()
                .context("Failed to get absolute path to selected TreeItem")?
                .to_string());
        }
        bail!("Failed to get selected TreeItem")
    }
}

impl<'a> Widget for &mut Explorer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Ok(tree) = Tree::new(self.tree_items.children()) {
            StatefulWidget::render(
                tree.block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(self.root_path.file_name.clone())
                        .border_style(Style::default().fg(self.component_state.get_active_color()))
                        .title_style(Style::default().fg(Color::Green))
                        .title_alignment(Alignment::Center),
                )
                .highlight_style(
                    Style::new()
                        .fg(Color::Black)
                        .bg(Color::Rgb(57, 59, 64))
                        .add_modifier(Modifier::BOLD),
                ),
                area,
                buf,
                &mut self.tree_state,
            );
        }
    }
}

impl<'a> Component for Explorer<'a> {
    fn handle_event(&mut self, event: Event) -> Result<Action> {
        if let Some(key_event) = event.as_key_event() {
            // Handle events here that should not be passed on to the vim emulation handler.
            match self.handle_key_events(key_event)? {
                Action::Handled => return Ok(Action::Handled),
                Action::OpenTab => return Ok(Action::OpenTab),
                _ => {}
            }
        }
        if let Some(mouse_event) = event.as_mouse_event()
            && let Action::Handled = self.handle_mouse_events(mouse_event)?
        {
            return Ok(Action::Handled);
        }
        Ok(Action::Pass)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action> {
        if key.code == KeyCode::Enter
            && let Ok(selected) = self.selected()
            && Path::new(&selected).is_file()
        {
            // Open a tab if the selected item is a file.
            return Ok(Action::OpenTab);
        }

        let changed = match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.tree_state.key_up(),
            KeyCode::Down | KeyCode::Char('j') => self.tree_state.key_down(),
            KeyCode::Left | KeyCode::Char('h') => {
                // Do not call key_left(); Calling it on a closed folder clears the selection.
                let key = self.tree_state.selected().to_owned();
                self.tree_state.close(key.as_ref())
            }
            KeyCode::Char(' ') | KeyCode::Enter => self
                .tree_state
                .toggle(self.tree_state.selected().to_owned()),
            KeyCode::Right | KeyCode::Char('l') => self.tree_state.key_right(),
            _ => false,
        };
        if changed {
            return Ok(Action::Handled);
        }
        Ok(Action::Noop)
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Action> {
        let changed = match mouse.kind {
            MouseEventKind::ScrollDown => self.tree_state.scroll_down(1),
            MouseEventKind::ScrollUp => self.tree_state.scroll_up(1),
            MouseEventKind::Down(_button) => self
                .tree_state
                .click_at(Position::new(mouse.column, mouse.row)),
            _ => false,
        };
        if changed {
            return Ok(Action::Handled);
        }
        Ok(Action::Noop)
    }

    fn is_active(&self) -> bool {
        self.component_state.focus == Focus::Active
    }
}
