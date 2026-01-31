use anyhow::{Context, Result, anyhow};
use clap::Parser;
use log::{info, trace};
use std::process::{Command, Stdio};

pub mod gui;
pub mod tui;
/// Extendable command-line driven development environment written in Rust using the Qt UI framework.
/// If no flags are provided, the GUI editor is launched in a separate process.
/// If no path is provided, the current directory is used.
#[derive(Parser, Debug)]
#[structopt(name = "clide", verbatim_doc_comment)]
struct Cli {
    /// The root directory for the project to open with the clide editor.
    #[arg(value_parser = clap::value_parser!(std::path::PathBuf))]
    pub path: Option<std::path::PathBuf>,

    /// Run clide in headless mode.
    #[arg(value_name = "tui", short, long)]
    pub tui: bool,

    /// Run the clide GUI in the current process, blocking the terminal and showing all output streams.
    #[arg(value_name = "gui", short, long)]
    pub gui: bool,
}

impl Cli {
    fn run_mode(&self) -> Result<RunMode> {
        let mut modes = Vec::new();
        self.tui.then(|| modes.push(RunMode::Tui));
        self.gui.then(|| modes.push(RunMode::GuiAttached));
        match &modes[..] {
            [] => Ok(RunMode::Gui),
            [mode] => Ok(*mode),
            multiple => Err(anyhow!(
                "More than one run mode found {multiple:?} please select one."
            )),
        }
    }
}

pub struct AppContext {
    pub path: std::path::PathBuf,
    pub run_mode: RunMode,
}

impl AppContext {
    fn new(cli: Cli) -> Result<Self> {
        let path = match &cli.path {
            // If the CLI was provided a directory, convert it to absolute.
            Some(path) => std::path::absolute(path)?,
            // If no path was provided, use the current directory.
            None => std::env::current_dir().context("Failed to obtain current directory")?,
        };
        info!(target:"main()", "Root path detected: {path:?}");

        Ok(Self {
            path,
            run_mode: cli.run_mode()?,
        })
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum RunMode {
    #[default]
    Gui,
    GuiAttached,
    Tui,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let app_context = AppContext::new(args)?;
    match app_context.run_mode {
        RunMode::GuiAttached => gui::run(app_context),
        RunMode::Tui => tui::run(app_context),
        RunMode::Gui => {
            trace!(target:"main()", "Starting GUI in a new process");
            Command::new(std::env::current_exe()?)
                .args(&["--gui", app_context.path.to_str().unwrap()])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .stdin(Stdio::null())
                .spawn()
                .context("Failed to start GUI")
                .map(|_| ())
        }
    }
}
