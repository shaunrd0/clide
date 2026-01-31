use crate::tui::component::{Action, Component, ComponentState, FocusState};
use crate::tui::menu_bar::MenuBarItemOption::{
    About, CloseTab, Exit, Reload, Save, ShowHideExplorer, ShowHideLogger,
};
use anyhow::Context;
use log::trace;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, Clear, List, ListItem, ListState, StatefulWidget, Tabs, Widget,
};
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter)]
enum MenuBarItem {
    File,
    View,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter)]
enum MenuBarItemOption {
    Save,
    CloseTab,
    Reload,
    Exit,
    ShowHideExplorer,
    ShowHideLogger,
    About,
}

impl MenuBarItemOption {
    fn id(&self) -> &str {
        match self {
            Save => "Save",
            Reload => "Reload",
            Exit => "Exit",
            ShowHideExplorer => "Show / hide explorer",
            ShowHideLogger => "Show / hide logger",
            About => "About",
            CloseTab => "Close tab",
        }
    }
}

impl MenuBarItem {
    pub fn next(self) -> Self {
        let cur = self as usize;
        let next = cur.saturating_add(1);
        Self::from_repr(next).unwrap_or(self)
    }

    pub fn prev(self) -> Self {
        let cur = self as usize;
        let prev = cur.saturating_sub(1);
        Self::from_repr(prev).unwrap_or(self)
    }

    pub fn id(&self) -> &str {
        match self {
            MenuBarItem::File => "File",
            MenuBarItem::View => "View",
            MenuBarItem::Help => "Help",
        }
    }

    pub fn options(&self) -> &[MenuBarItemOption] {
        match self {
            MenuBarItem::File => &[Save, CloseTab, Reload, Exit],
            MenuBarItem::View => &[ShowHideExplorer, ShowHideLogger],
            MenuBarItem::Help => &[About],
        }
    }
}

pub struct MenuBar {
    selected: MenuBarItem,
    opened: Option<MenuBarItem>,
    pub(crate) component_state: ComponentState,
    list_state: ListState,
}

impl MenuBar {
    pub const ID: &str = "MenuBar";

    const DEFAULT_HELP: &str = "(←/h)/(→/l): Select option | Enter: Choose selection";
    pub fn new() -> Self {
        trace!(target:Self::ID, "Building {}", Self::ID);
        Self {
            selected: MenuBarItem::File,
            opened: None,
            component_state: ComponentState::default().with_help_text(Self::DEFAULT_HELP),
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let titles: Vec<Line> = MenuBarItem::iter()
            .map(|item| Line::from(item.id().to_owned()))
            .collect();
        let tabs_style = Style::default();
        let highlight_style = if self.opened.is_some() {
            Style::default().bg(Color::Blue).fg(Color::White)
        } else {
            Style::default().bg(Color::Cyan).fg(Color::Black)
        };
        Tabs::new(titles)
            .style(tabs_style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.component_state.get_active_color())),
            )
            .highlight_style(highlight_style)
            .select(self.selected as usize)
            .render(area, buf);
    }

    fn render_drop_down(
        &mut self,
        title_bar_anchor: Rect,
        area: Rect,
        buf: &mut Buffer,
        opened: MenuBarItem,
    ) {
        let popup_area = Self::rect_under_option(title_bar_anchor, area, 27, 10);
        Clear::default().render(popup_area, buf);
        let options = opened.options().iter().map(|i| ListItem::new(i.id()));
        StatefulWidget::render(
            List::new(options)
                .block(Block::bordered().title(self.selected.id()))
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> "),
            popup_area,
            buf,
            &mut self.list_state,
        );
    }

    fn rect_under_option(anchor: Rect, area: Rect, width: u16, height: u16) -> Rect {
        let rect = Rect {
            x: anchor.x,
            y: anchor.y + anchor.height,
            width: width.min(area.width),
            height,
        };
        // TODO: X offset for item option? It's fine as-is, but it might look nicer.
        // trace!(target:Self::ID, "Building Rect under MenuBar popup {}", rect);
        rect
    }
}

impl Widget for &mut MenuBar {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title_bar_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 3,
        };
        self.render_title_bar(title_bar_area, buf);
        if let Some(opened) = self.opened {
            self.render_drop_down(title_bar_area, area, buf, opened);
        }
    }
}

impl Component for MenuBar {
    fn handle_key_events(&mut self, key: KeyEvent) -> anyhow::Result<Action> {
        if self.opened.is_some() {
            // Keybinds for popup menu.
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.list_state.select_previous();
                    Ok(Action::Handled)
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.list_state.select_next();
                    Ok(Action::Handled)
                }
                KeyCode::Enter => {
                    if let Some(selected) = self.list_state.selected() {
                        let selection = self
                            .selected
                            .options()
                            .get(selected)
                            .context("Failed to get selected MenuBar option")?;
                        return match selection {
                            Save => Ok(Action::Save),
                            Exit => Ok(Action::Quit),
                            Reload => Ok(Action::ReloadFile),
                            ShowHideExplorer => Ok(Action::ShowHideExplorer),
                            ShowHideLogger => Ok(Action::ShowHideLogger),
                            About => Ok(Action::ShowHideAbout),
                            CloseTab => Ok(Action::CloseTab),
                        };
                    }
                    Ok(Action::Noop)
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.opened = None;
                    self.component_state.help_text = Self::DEFAULT_HELP.to_string();
                    self.list_state.select_first();
                    Ok(Action::Handled)
                }
                _ => Ok(Action::Noop),
            }
        } else {
            // Keybinds for title bar.
            match key.code {
                KeyCode::Left | KeyCode::Char('h') => {
                    self.selected = self.selected.prev();
                    Ok(Action::Handled)
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.selected = self.selected.next();
                    Ok(Action::Handled)
                }
                KeyCode::Enter => {
                    self.opened = Some(self.selected);
                    self.component_state.help_text = concat!(
                        "(↑/k)/(↓/j): Select option | Enter: Choose selection |",
                        " ESC/Q: Close drop-down menu"
                    )
                    .to_string();
                    Ok(Action::Handled)
                }
                _ => Ok(Action::Noop),
            }
        }
    }
}
