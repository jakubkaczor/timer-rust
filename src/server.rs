//! Items used exclusively by the server.

use crate::Request;
use anyhow::Result;
use async_std::{
    os::unix::net::{UnixListener, UnixStream},
    prelude::*,
    task,
};
use log::{info, warn};

pub async fn handle_requests(listener: UnixListener) -> Result<()> {
    let mut incoming = listener.incoming();
    while let Some(connection) = incoming.next().await {
        if let Ok(stream) = connection {
            task::spawn(handle_request(stream));
        } else {
            warn!("Failed accepting a connection.");
            continue;
        }
    }
    Ok(())
}

async fn handle_request(mut stream: UnixStream) -> Result<()> {
    let mut buf = Vec::new();

    stream.read_to_end(&mut buf).await.map_err(|err| {
        warn!("Reading to buffer failed.");
        err
    })?;

    let request = serde_json::from_slice(&buf).map_err(|err| {
        warn!("Deserializing a request failed.");
        err
    })?;

    info!("Handling request: {:?}.", request);
    match request {
        Request::Add(timer) => {
            let duration = timer.duration();
            info!("Sleeping for {:?}.", duration);
            task::sleep(duration).await;
            info!("Woke up!");
        }
        Request::Dump => todo!(),
    }
    Ok(())
}
