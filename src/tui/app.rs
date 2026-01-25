use crate::tui::about::About;
use crate::tui::app::AppComponent::{AppEditor, AppExplorer, AppLogger};
use crate::tui::component::{Action, Component, Focus, FocusState, Visibility, VisibleState};
use crate::tui::editor_tab::EditorTab;
use crate::tui::explorer::Explorer;
use crate::tui::logger::Logger;
use crate::tui::menu_bar::MenuBar;
use AppComponent::AppMenuBar;
use anyhow::{Context, Result};
use log::{error, info, trace};
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Widget};
use ratatui::widgets::{Paragraph, Wrap};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppComponent {
    AppEditor,
    AppExplorer,
    AppLogger,
    AppMenuBar,
}

pub struct App<'a> {
    editor_tabs: EditorTab,
    explorer: Explorer<'a>,
    logger: Logger,
    menu_bar: MenuBar,
    last_active: AppComponent,
    about: bool,
}

impl<'a> App<'a> {
    pub fn id() -> &'static str {
        "App"
    }

    pub fn new(root_path: PathBuf) -> Result<Self> {
        trace!(target:Self::id(), "Building {}", Self::id());
        let app = Self {
            editor_tabs: EditorTab::new(None),
            explorer: Explorer::new(&root_path)?,
            logger: Logger::new(),
            menu_bar: MenuBar::new(),
            last_active: AppEditor,
            about: false,
        };
        Ok(app)
    }

    /// Logic that should be executed once on application startup.
    pub fn start(&mut self) -> Result<()> {
        trace!(target:Self::id(), "Starting App");
        Ok(())
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.start()?;
        trace!(target:Self::id(), "Entering App run loop");
        loop {
            terminal.draw(|f| {
                f.render_widget(&mut self, f.area());
            })?;

            if event::poll(Duration::from_millis(250)).context("event poll failed")? {
                match self.handle_event(event::read()?)? {
                    Action::Quit => break,
                    Action::Handled => {}
                    _ => {
                        // bail!("Unhandled event: {:?}", event);
                    }
                }
            }
        }
        Ok(())
    }

    fn draw_bottom_status(&self, area: Rect, buf: &mut Buffer) {
        // Determine help text from the most recently focused component.
        let help = match self.last_active {
            AppEditor => match self.editor_tabs.current_editor() {
                Some(editor) => editor.component_state.help_text.clone(),
                None => {
                    if !self.editor_tabs.is_empty() {
                        error!(target:Self::id(), "Failed to get Editor while drawing bottom status bar");
                    }
                    "Failed to get current Editor while getting widget help text".to_string()
                }
            },
            AppExplorer => self.explorer.component_state.help_text.clone(),
            AppLogger => self.logger.component_state.help_text.clone(),
            AppMenuBar => self.menu_bar.component_state.help_text.clone(),
        };
        Paragraph::new(
            concat!(
                "ALT+Q: Focus project explorer | ALT+W: Focus editor | ALT+E: Focus logger |",
                " ALT+R: Focus menu bar | CTRL+C: Quit\n"
            )
            .to_string()
                + help.as_str(),
        )
        .style(Color::Gray)
        .wrap(Wrap { trim: false })
        .centered()
        .render(area, buf);
    }

    fn clear_focus(&mut self) {
        info!(target:Self::id(), "Clearing all widget focus");
        self.explorer.component_state.set_focus(Focus::Inactive);
        self.explorer.component_state.set_focus(Focus::Inactive);
        self.logger.component_state.set_focus(Focus::Inactive);
        self.menu_bar.component_state.set_focus(Focus::Inactive);
        match self.editor_tabs.current_editor_mut() {
            None => {
                error!(target:Self::id(), "Failed to get current Editor while clearing focus")
            }
            Some(editor) => editor.component_state.set_focus(Focus::Inactive),
        }
    }

    fn change_focus(&mut self, focus: AppComponent) {
        info!(target:Self::id(), "Changing widget focus to {:?}", focus);
        self.clear_focus();
        match focus {
            AppEditor => match self.editor_tabs.current_editor_mut() {
                None => {
                    error!(target:Self::id(), "Failed to get current Editor while changing focus")
                }
                Some(editor) => editor.component_state.set_focus(Focus::Active),
            },
            AppExplorer => self.explorer.component_state.set_focus(Focus::Active),
            AppLogger => self.logger.component_state.set_focus(Focus::Active),
            AppMenuBar => self.menu_bar.component_state.set_focus(Focus::Active),
        }
        self.last_active = focus;
    }

    /// Refresh the contents of the editor to match the selected TreeItem in the file Explorer.
    /// If the selected item is not a file, this does nothing.
    #[allow(unused)]
    fn refresh_editor_contents(&mut self) -> Result<()> {
        // TODO: This may be useful for a preview mode of the selected file prior to opening a tab.
        // Use the currently selected TreeItem or get an absolute path to this source file.
        // let selected_pathbuf = match self.explorer.selected() {
        //     Ok(path) => PathBuf::from(path),
        //     Err(_) => PathBuf::from(std::path::absolute(file!())?.to_string_lossy().to_string()),
        // };
        // match self.editor_tabs.current_editor_mut() {
        //     None => bail!("Failed to get current Editor while refreshing editor contents"),
        //     Some(editor) => {
        //         let current_file_path = editor
        //             .file_path
        //             .clone()
        //             .context("Failed to get Editor current file_path")?;
        //         if selected_pathbuf == current_file_path || !selected_pathbuf.is_file() {
        //             return Ok(());
        //         }
        //         editor.set_contents(&selected_pathbuf)
        //     }
        // }
        Ok(())
    }
}

impl<'a> Widget for &mut App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let vertical_constraints = match self.logger.component_state.vis {
            Visibility::Visible => {
                vec![
                    Constraint::Length(3),      // top status bar
                    Constraint::Percentage(70), // horizontal layout
                    Constraint::Fill(1),        // terminal
                    Constraint::Length(3),      // bottom status bar
                ]
            }
            Visibility::Hidden => {
                vec![
                    Constraint::Length(3), // top status bar
                    Constraint::Fill(1),   // horizontal layout
                    Constraint::Length(3), // bottom status bar
                ]
            }
        };
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vertical_constraints)
            .split(area);

        let horizontal_constraints = match self.explorer.component_state.vis {
            Visibility::Visible => {
                vec![
                    Constraint::Max(30), // File explorer with a max width of 30 characters.
                    Constraint::Fill(1), // Editor fills the remaining space.
                ]
            }
            Visibility::Hidden => {
                vec![
                    Constraint::Fill(1), // Editor fills the remaining space.
                ]
            }
        };

        // The index used for vertical here does not care if the Logger is Visible or not.
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(horizontal_constraints)
            .split(vertical[1]);
        match self.explorer.component_state.vis {
            Visibility::Visible => {
                let editor_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1), // Editor tabs.
                        Constraint::Fill(1),   // Editor contents.
                    ])
                    .split(horizontal[1]);
                self.editor_tabs
                    .render(editor_layout[0], editor_layout[1], buf);
                self.explorer.render(horizontal[0], buf);
            }
            Visibility::Hidden => {
                let editor_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1), // Editor tabs.
                        Constraint::Fill(1),   // Editor contents.
                    ])
                    .split(horizontal[0]);
                self.editor_tabs
                    .render(editor_layout[0], editor_layout[1], buf);
            }
        }

        match self.logger.component_state.vis {
            // Index 1 of vertical is rendered with the horizontal layout above.
            Visibility::Visible => {
                self.logger.render(vertical[2], buf);
                self.draw_bottom_status(vertical[3], buf);
                // The title bar is rendered last to overlay any popups created for drop-down menus.
                self.menu_bar.render(vertical[0], buf);
            }
            Visibility::Hidden => {
                self.draw_bottom_status(vertical[2], buf);
                // The title bar is rendered last to overlay any popups created for drop-down menus.
                self.menu_bar.render(vertical[0], buf);
            }
        }

        if self.about {
            let about_area = area.centered(Constraint::Percentage(50), Constraint::Percentage(45));
            About::new().render(about_area, buf);
        }
    }
}

impl<'a> Component for App<'a> {
    /// Handles events for the App and delegates to attached Components.
    fn handle_event(&mut self, event: Event) -> Result<Action> {
        // Handle events in the primary application.
        if let Some(key_event) = event.as_key_event() {
            let res = self
                .handle_key_events(key_event)
                .context("Failed to handle key events for primary App Component.");
            match res {
                Ok(Action::Quit) | Ok(Action::Handled) => return res,
                _ => {}
            }
        }
        // Handle events for all components.
        let action = match self.last_active {
            AppEditor => self.editor_tabs.handle_event(event.clone())?,
            AppExplorer => self.explorer.handle_event(event.clone())?,
            AppLogger => self.logger.handle_event(event.clone())?,
            AppMenuBar => self.menu_bar.handle_event(event.clone())?,
        };

        // Components should always handle mouse events for click interaction.
        if let Some(mouse) = event.as_mouse_event() {
            if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                if let Some(editor) = self.editor_tabs.current_editor_mut() {
                    editor.handle_mouse_events(mouse)?;
                }
                self.explorer.handle_mouse_events(mouse)?;
                self.logger.handle_mouse_events(mouse)?;
            }
        }

        // Handle actions returned from widgets that may need context on other widgets or app state.
        match action {
            Action::Quit | Action::Handled => Ok(action),
            Action::Save => match self.editor_tabs.current_editor_mut() {
                None => {
                    error!(target:Self::id(), "Failed to get current editor while handling App Action::Save");
                    Ok(Action::Noop)
                }
                Some(editor) => match editor.save() {
                    Ok(_) => Ok(Action::Handled),
                    Err(e) => {
                        error!(target:Self::id(), "Failed to save editor contents: {e}");
                        Ok(Action::Noop)
                    }
                },
            },
            Action::OpenTab => {
                if let Ok(path) = self.explorer.selected() {
                    let path_buf = PathBuf::from(path);
                    self.editor_tabs.open_tab(&path_buf)?;
                    Ok(Action::Handled)
                } else {
                    Ok(Action::Noop)
                }
            }
            Action::CloseTab => match self.editor_tabs.close_current_tab() {
                Ok(_) => Ok(Action::Handled),
                Err(_) => Ok(Action::Noop),
            },
            Action::ReloadFile => {
                trace!(target:Self::id(), "Reloading file for current editor");
                if let Some(editor) = self.editor_tabs.current_editor_mut() {
                    editor
                        .reload_contents()
                        .map(|_| Action::Handled)
                        .context("Failed to handle Action::ReloadFile")
                } else {
                    error!(target:Self::id(), "Failed to get current editor while handling App Action::ReloadFile");
                    Ok(Action::Noop)
                }
            }
            Action::ShowHideLogger => {
                self.logger.component_state.togget_visible();
                Ok(Action::Handled)
            }
            Action::ShowHideExplorer => {
                self.explorer.component_state.togget_visible();
                Ok(Action::Handled)
            }
            Action::ShowHideAbout => {
                self.about = !self.about;
                Ok(Action::Handled)
            }
            _ => Ok(Action::Noop),
        }
    }

    /// Handles key events for the App Component only.
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action> {
        match key.code {
            // If the ESC key is pressed with the About page open, hide it.
            KeyCode::Esc | KeyCode::Char('q') => {
                if self.about {
                    self.about = false;
                    return Ok(Action::Handled);
                }
            }
            _ => {}
        }

        match key {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: _state,
            } => {
                self.change_focus(AppExplorer);
                Ok(Action::Handled)
            }
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: _state,
            } => {
                self.change_focus(AppEditor);
                Ok(Action::Handled)
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: _state,
            } => {
                self.change_focus(AppLogger);
                Ok(Action::Handled)
            }
            KeyEvent {
                code: KeyCode::Char('r'),
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: _state,
            } => {
                self.change_focus(AppMenuBar);
                Ok(Action::Handled)
            }
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _state,
            } => Ok(Action::Quit),
            _ => Ok(Action::Noop),
        }
    }
}
