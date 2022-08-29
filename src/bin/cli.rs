//! Command-line interface client.

use anyhow::{Context, Result};
use clap::Parser;
use serde_json;
use std::os::unix::net::UnixStream;
use timer::{
    cli::{Cli, Subcommand},
    Request, Timer, SOCKET_PATH,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let request = match cli.subcommand {
        Subcommand::Add { time } => Request::Add(Timer::from_secs(time)),
        _ => todo!(),
    };

    let stream = UnixStream::connect(SOCKET_PATH)
        .context("Unable to connect to the socket. Is server running?")?;
    serde_json::to_writer(stream, &request)
        .context("Deserializing or writing request to the stream failed.")?;

    Ok(())
}
