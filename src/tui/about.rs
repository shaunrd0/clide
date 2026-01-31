use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Widget, Wrap};

pub struct About {}

impl About {
    #[allow(unused)]
    pub const ID: &str = "About";

    pub fn new() -> Self {
        // trace!(target:Self::id(), "Building {}", Self::id());
        Self {}
    }
}

impl Widget for About {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1), // Image Layout
                Constraint::Fill(2), // Description
            ])
            .split(area);
        let kilroy = [
            "    *         ",
            "    |.===.    ",
            "    {}o o{}   ",
            "-ooO--(_)--Ooo",
            "CLIDE WAS HERE",
        ];
        let kilroy_rect = Rect {
            x: chunks[1].x,
            y: chunks[1].y - kilroy.len() as u16 + 2,
            width: area.width,
            height: kilroy.len() as u16,
        };
        // info!(target: About::ID, "Created rect: {kilroy_rect:?}");

        let kilroy_lines: Vec<Line> = kilroy.iter().map(|l| Line::from(Span::raw(*l))).collect();
        let about_text = [
            "Clide",
            "",
            "Author: Shaun Reed",
            "Email: shaunrd0@gmail.com",
            "URL: https://git.shaunreed.com/shaunrd0/clide",
            "Blog: https://shaunreed.com",
            "",
            "Description:",
            concat!(
                "CLIDE is an extendable command-line driven development environment written in Rust",
                " using the Qt UI framework that supports both full and headless Linux environments.",
                " The GUI is written in QML compiled through Rust using the cxx-qt crate, while the",
                " TUI was implemented using the ratatui crate.",
            ),
        ];
        let about_lines: Vec<Line> = about_text
            .iter()
            .map(|l| Line::from(Span::raw(*l)))
            .collect();

        Clear::default().render(kilroy_rect, buf);
        Clear::default().render(chunks[1], buf);
        Paragraph::new(about_lines)
            .block(
                Block::default()
                    .title("About")
                    .borders(Borders::ALL)
                    .padding(Padding::top(0)),
            )
            .wrap(Wrap { trim: false })
            .render(chunks[1], buf);
        Paragraph::new(kilroy_lines)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .padding(Padding::bottom(0)),
            )
            .wrap(Wrap { trim: false })
            .centered()
            .render(kilroy_rect, buf);
    }
}
