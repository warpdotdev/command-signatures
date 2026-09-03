//! A thin CLI over `warp_command_signatures`'s reusable listing API.

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use warp_command_signatures::{list_signatures, SignatureSource, SignatureSummary};

#[derive(Parser)]
#[command(
    name = "command-signatures",
    about = "Utilities for working with Warp command completion signatures"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List command signatures as a text table or a JSON array.
    List {
        /// Read signatures from this Fig-compatible JSON file instead of the embedded
        /// repository signatures. Replaces the embedded source rather than merging with it.
        #[arg(long)]
        file: Option<PathBuf>,

        /// Emit a JSON array instead of a text table.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::List { file, json } => run_list(file, json),
    }
}

fn run_list(file: Option<PathBuf>, json: bool) -> ExitCode {
    let source = match file {
        Some(path) => SignatureSource::File(path),
        None => SignatureSource::Embedded,
    };

    let summaries = match list_signatures(source) {
        Ok(summaries) => summaries,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };

    let render_result = if json {
        render_json(&summaries)
    } else {
        render_text(&summaries)
    };

    // A closed output pipe (e.g. piping into `head`) should terminate cleanly rather than
    // panicking on a broken-pipe write error.
    match render_result {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::SUCCESS,
    }
}

fn render_text(summaries: &[SignatureSummary]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    if summaries.is_empty() {
        return writeln!(out, "No signatures found.");
    }

    writeln!(out, "NAME\tSUBCOMMANDS\tDESCRIPTION")?;
    for summary in summaries {
        let description = summary
            .description
            .as_deref()
            .map(normalize_description)
            .unwrap_or_default();
        writeln!(
            out,
            "{}\t{}\t{description}",
            summary.name, summary.subcommand_count
        )?;
    }
    Ok(())
}

/// Replaces tabs, carriage returns, and newlines with spaces so each signature occupies exactly
/// one line of text output.
fn normalize_description(description: &str) -> String {
    description
        .chars()
        .map(|c| match c {
            '\t' | '\r' | '\n' => ' ',
            other => other,
        })
        .collect()
}

fn render_json(summaries: &[SignatureSummary]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer(&mut out, summaries).map_err(io::Error::from)?;
    writeln!(out)
}
