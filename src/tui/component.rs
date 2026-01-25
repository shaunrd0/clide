#![allow(dead_code, unused_variables)]

use crate::tui::component::Focus::Inactive;
use Focus::Active;
use anyhow::Result;
use log::trace;
use ratatui::crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::style::Color;

pub enum Action {
    /// Exit the application.
    Quit,

    /// The input was checked by the Component and had no effect.
    Noop,

    /// Pass input to another component or external handler.
    /// Similar to Noop with the added context that externally handled input may have had an impact.
    Pass,

    /// Save the current file.
    Save,

    /// The input was handled by a Component and should not be passed to the next component.
    Handled,
    OpenTab,
    ReloadFile,
    ShowHideExplorer,
    ShowHideLogger,
    ShowHideAbout,
    CloseTab,
}

pub trait Component {
    fn handle_event(&mut self, event: Event) -> Result<Action> {
        match event {
            Event::Key(key_event) => self.handle_key_events(key_event),
            _ => Ok(Action::Noop),
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Action> {
        Ok(Action::Noop)
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Action> {
        Ok(Action::Noop)
    }

    fn update(&mut self, action: Action) -> Result<Action> {
        Ok(Action::Noop)
    }

    /// Override this method for creating components that conditionally handle input.
    fn is_active(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct ComponentState {
    pub(crate) focus: Focus,
    pub(crate) vis: Visibility,
    pub(crate) help_text: String,
}

impl ComponentState {
    pub fn id() -> &'static str {
        "ComponentState"
    }

    fn new() -> Self {
        trace!(target:Self::id(), "Building {}", Self::id());
        Self {
            focus: Active,
            vis: Visibility::Visible,
            help_text: String::new(),
        }
    }

    pub(crate) fn with_help_text(mut self, help_text: &str) -> Self {
        self.help_text = help_text.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Focus {
    Active,
    #[default]
    Inactive,
}

impl Focus {
    pub(crate) fn get_active_color(&self) -> Color {
        match self {
            Active => Color::LightYellow,
            Inactive => Color::White,
        }
    }
}

pub trait FocusState {
    fn with_focus(self, focus: Focus) -> Self;
    fn set_focus(&mut self, focus: Focus);
    fn toggle_focus(&mut self);
    fn get_active_color(&self) -> Color;
}

impl FocusState for ComponentState {
    fn with_focus(self, focus: Focus) -> Self {
        Self {
            focus,
            vis: Visibility::Visible,
            help_text: self.help_text,
        }
    }

    fn set_focus(&mut self, focus: Focus) {
        self.focus = focus;
    }

    fn toggle_focus(&mut self) {
        match self.focus {
            Active => self.set_focus(Inactive),
            Inactive => self.set_focus(Active),
        }
    }

    fn get_active_color(&self) -> Color {
        self.focus.get_active_color()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Visibility {
    #[default]
    Visible,
    Hidden,
}

pub trait VisibleState {
    fn with_visible(self, vis: Visibility) -> Self;
    fn set_visible(&mut self, vis: Visibility);
    fn togget_visible(&mut self);
}

impl VisibleState for ComponentState {
    fn with_visible(self, vis: Visibility) -> Self {
        Self {
            focus: self.focus,
            vis,
            help_text: self.help_text,
        }
    }

    fn set_visible(&mut self, vis: Visibility) {
        self.vis = vis;
    }

    fn togget_visible(&mut self) {
        match self.vis {
            Visibility::Visible => self.set_visible(Visibility::Hidden),
            Visibility::Hidden => self.set_visible(Visibility::Visible),
        }
    }
}
