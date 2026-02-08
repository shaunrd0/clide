// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

use crate::tui::component::{Action, Component, ComponentState, Focus, FocusState};
use log::{LevelFilter, trace};
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetEvent, TuiWidgetState};

/// Any log written as info!(target:self.id(), "message") will work with this logger.
/// The logger is bound to info!, debug!, error!, trace! macros within Tui::new().
pub struct Logger {
    state: TuiWidgetState,
    pub(crate) component_state: ComponentState,
}

impl Logger {
    pub const ID: &str = "Logger";

    pub fn new() -> Self {
        trace!(target:Self::ID, "Building {}", Self::ID);
        let state = TuiWidgetState::new();
        state.transition(TuiWidgetEvent::HideKey);
        Self {
            state: state
                .set_level_for_target("arboard::platform::linux::x11", LevelFilter::Off)
                .set_level_for_target("mio::poll", LevelFilter::Off),
            component_state: ComponentState::default().with_help_text(concat!(
                "Space: Hide/show logging target selector panel | (↑/k)/(↓/j): Select target |",
                " (←/h)/(→/l): Display level | f: Focus target | +/-: Filter level |",
                " v: Toggle filtered targets visibility | PageUp/Down: Scroll | Esc: Cancel scroll"
            )),
        }
    }
}

impl Widget for &Logger {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        TuiLoggerSmartWidget::default()
            .border_style(Style::default().fg(self.component_state.get_active_color()))
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .state(&self.state)
            .render(area, buf);
    }
}

impl Component for Logger {
    fn handle_event(&mut self, event: Event) -> anyhow::Result<Action> {
        if let Some(key_event) = event.as_key_event() {
            return self.handle_key_events(key_event);
        }
        Ok(Action::Noop)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> anyhow::Result<Action> {
        match key.code {
            KeyCode::Char('v') => self.state.transition(TuiWidgetEvent::SpaceKey),
            KeyCode::Esc => self.state.transition(TuiWidgetEvent::EscapeKey),
            KeyCode::PageUp => self.state.transition(TuiWidgetEvent::PrevPageKey),
            KeyCode::PageDown => self.state.transition(TuiWidgetEvent::NextPageKey),
            KeyCode::Up | KeyCode::Char('k') => self.state.transition(TuiWidgetEvent::UpKey),
            KeyCode::Down | KeyCode::Char('j') => self.state.transition(TuiWidgetEvent::DownKey),
            KeyCode::Left | KeyCode::Char('h') => self.state.transition(TuiWidgetEvent::LeftKey),
            KeyCode::Right | KeyCode::Char('l') => self.state.transition(TuiWidgetEvent::RightKey),
            KeyCode::Char('+') => self.state.transition(TuiWidgetEvent::PlusKey),
            KeyCode::Char('-') => self.state.transition(TuiWidgetEvent::MinusKey),
            KeyCode::Char(' ') => self.state.transition(TuiWidgetEvent::HideKey),
            KeyCode::Char('f') => self.state.transition(TuiWidgetEvent::FocusKey),
            _ => (),
        }
        Ok(Action::Pass)
    }

    fn is_active(&self) -> bool {
        self.component_state.focus == Focus::Active
    }
}
