mod about;
mod app;
mod component;
mod editor;
mod editor_tab;
mod explorer;
mod logger;
mod menu_bar;

use anyhow::{Context, Result};
use log::{LevelFilter, debug, info, trace};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
};
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::env;
use std::io::{Stdout, stdout};
use tui_logger::{
    TuiLoggerFile, TuiLoggerLevelOutput, init_logger, set_default_level, set_log_file,
};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    root_path: std::path::PathBuf,
}

impl Tui {
    pub fn id() -> &'static str {
        "Tui"
    }

    pub fn new(root_path: std::path::PathBuf) -> Result<Self> {
        trace!(target:Self::id(), "Building {}", Self::id());
        init_logger(LevelFilter::Trace)?;
        set_default_level(LevelFilter::Trace);
        debug!(target:Self::id(), "Logging initialized");

        let mut dir = env::temp_dir();
        dir.push("clide.log");
        let file_options = TuiLoggerFile::new(
            dir.to_str()
                .context("Failed to set temp directory for file logging")?,
        )
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_file(false)
        .output_separator(':');
        set_log_file(file_options);
        debug!(target:Self::id(), "Logging to file: {dir:?}");

        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            root_path,
        })
    }

    pub fn start(self) -> Result<()> {
        info!(target:Self::id(), "Starting the TUI editor at {:?}", self.root_path);
        ratatui::crossterm::execute!(
            stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            EnableBracketedPaste
        )?;
        enable_raw_mode()?;

        let app_result = app::App::new(self.root_path)?
            .run(self.terminal)
            .context("Failed to start the TUI editor.");
        Self::stop()?;
        app_result
    }

    fn stop() -> Result<()> {
        info!(target:Self::id(), "Stopping the TUI editor");
        disable_raw_mode()?;
        ratatui::crossterm::execute!(
            stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            DisableBracketedPaste
        )?;
        Ok(())
    }
}
