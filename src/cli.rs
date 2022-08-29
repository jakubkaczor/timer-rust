//! Items used exclusively by the CLI client.

use clap;

/// Command-line timer.
#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Add a new timer.
    #[clap(visible_alias("a"))]
    Add {
        #[clap(value_parser)]
        time: u64,
    },

    /// Print a snapshot of all active timers.
    #[clap(visible_aliases(&["snap", "s"]))]
    Snapshot,

    /// Watch timers progress.
    #[clap(visible_alias("w"))]
    Watch,
}
