use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Widget, Wrap};

pub struct About {}

impl About {
    #[allow(unused)]
    pub fn id() -> &'static str {
        "About"
    }

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
        Clear::default().render(area, buf);
        // Split main area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(2), // image column
                Constraint::Fill(1), // image column
                Constraint::Fill(2), // text column
            ])
            .split(area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(3),
                Constraint::Fill(1),
            ])
            .split(chunks[1]);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(3),
                Constraint::Fill(1),
            ])
            .split(chunks[2]);

        // ---------- IMAGE ----------
        let kilroy_art = [
            "                          *                                    ",
            "                          |.===.                               ",
            "                          {}o o{}                              ",
            "-----------------------ooO--(_)--Ooo---------------------------",
            "#                                                             #",
            "#                     CLIDE WAS HERE                          #",
            "#                                                             #",
            "#          https://git.shaunreed.com/shaunred/clide           #",
            "#          https://shaunreed.com/shaunred/clide               #",
            "#                                                             #",
        ];

        let kilroy_lines: Vec<Line> = kilroy_art
            .iter()
            .map(|l| Line::from(Span::raw(*l)))
            .collect();

        Paragraph::new(kilroy_lines)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .padding(Padding::bottom(0)),
            )
            .wrap(Wrap { trim: false })
            .centered()
            .render(top_chunks[1], buf);

        // ---------- TEXT ----------
        let about_text = vec![
            Line::from(vec![Span::styled(
                "clide\n",
                Style::default().add_modifier(Modifier::BOLD),
            )])
            .centered(),
            Line::from(""),
            Line::from(vec![
                Span::styled("Author: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Shaun Reed"),
            ])
            .left_aligned(),
            Line::from(vec![
                Span::styled("Email:  ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("shaunrd0@gmail.com"),
            ])
            .left_aligned(),
            Line::from(vec![
                Span::styled("URL:    ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("https://git.shaunreed.com/shaunrd0/clide"),
            ])
            .left_aligned(),
            Line::from(vec![
                Span::styled("Blog:   ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("https://shaunreed.com"),
            ])
            .left_aligned(),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Description\n",
                Style::default().add_modifier(Modifier::BOLD),
            )])
            .left_aligned(),
            Line::from(concat!(
                "CLIDE is an extendable command-line driven development environment written in Rust using the Qt UI framework that supports both full and headless Linux environments. ",
                "The GUI is written in QML compiled through Rust using the cxx-qt crate, while the TUI was implemented using the ratatui crate. ",
            ))
            .style(Style::default())
            .left_aligned(),
        ];
        Block::bordered().render(area, buf);

        let paragraph = Paragraph::new(about_text)
            .block(
                Block::default()
                    .title("About")
                    .borders(Borders::ALL)
                    .padding(Padding::top(0)),
            )
            .wrap(Wrap { trim: true });

        paragraph.render(bottom_chunks[1], buf);
    }
}
