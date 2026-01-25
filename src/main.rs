use crate::tui::Tui;
use anyhow::{Context, Result};
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

fn main() -> Result<()> {
    let args = Cli::parse();

    let root_path = match args.path {
        // If the CLI was provided a directory, convert it to absolute.
        Some(path) => std::path::absolute(path)?,
        // If no path was provided, use the current directory.
        None => std::env::current_dir().unwrap_or(
            // If we can't find the CWD, attempt to open the home directory.
            dirs::home_dir().context("Failed to obtain home directory")?,
        ),
    };
    info!(target:"main()", "Root path detected: {root_path:?}");

    match args.gui {
        true => {
            trace!(target:"main()", "Starting GUI");
            gui::run(root_path)
        }
        false => match args.tui {
            // Open the TUI editor if requested, otherwise use the QML GUI by default.
            true => {
                trace!(target:"main()", "Starting TUI");
                Ok(Tui::new(root_path)?.start()?)
            }
            false => {
                trace!(target:"main()", "Starting GUI in a new process");
                Command::new(std::env::current_exe()?)
                    .args(&["--gui", root_path.to_str().unwrap()])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .stdin(Stdio::null())
                    .spawn()?;
                Ok(())
            }
        },
    }
}
