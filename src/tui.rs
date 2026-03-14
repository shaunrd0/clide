// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

mod about;
mod app;
mod component;
mod editor;
mod editor_tab;
mod explorer;
mod logger;
mod menu_bar;

use crate::AppContext;
use anyhow::{Context, Result};
use libclide::log::Loggable;
use log::LevelFilter;
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

#[derive(Loggable)]
struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    root_path: std::path::PathBuf,
}

pub fn run(app_context: AppContext) -> Result<()> {
    libclide::trace!(target: "clide::tui::run", "Starting TUI");
    Tui::new(app_context)?.start()
}

impl Tui {
    fn new(app_context: AppContext) -> Result<Self> {
        init_logger(LevelFilter::Trace)?;
        set_default_level(LevelFilter::Trace);
        libclide::debug!("Logging initialized");

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
        libclide::debug!("Logging to file: {dir:?}");

        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            root_path: app_context.path,
        })
    }

    fn start(self) -> Result<()> {
        libclide::info!("Starting the TUI editor at {:?}", self.root_path);
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
        libclide::info!("Stopping the TUI editor");
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
