use anyhow::{Context, Result};
use async_std::{fs, os::unix::net::UnixListener};
use log::info;
use timer::server;
use timer::SOCKET_PATH;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    // The scope of the Result is covered by binding to address.
    #[allow(unused_must_use)]
    {
        fs::remove_file(SOCKET_PATH).await;
    }

    info!("Binding {:?} socket.", SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH)
        .await
        .context("Binding to socket failed.")?;

    info!("Handling requests...");
    server::handle_requests(listener)
        .await
        .context("Handling requests completely failed.")?;

    Ok(())
}
