use crate::{async_manager, chat, error::*};
use futures::{future::RemoteHandle, FutureExt, Sink, SinkExt, StreamExt};
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{iter, sync::Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        handshake::server::{Request, Response},
        http::StatusCode,
        Message,
    },
    WebSocketStream,
};
use tracing::{debug, warn};

#[derive(Serialize)]
pub struct ConnectionArgs {
    pub port: u16,
    pub path: String,
}

lazy_static! {
    static ref CURRENT_LISTEN_FUTURE: Mutex<Option<RemoteHandle<()>>> = Default::default();
}

#[tracing::instrument]
pub async fn start() -> Result<ConnectionArgs> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    debug!("listening on port {}", port);

    let mut rng = thread_rng();
    let path: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(32)
        .collect();

    {
        let path = path.clone();

        // TODO somehow kill these loops after inactivity (0 connected, 1+ minutes)
        // maybe only allow 1 server at a time?
        let (f, remote_handle) = async move {
            while let Ok((stream, _)) = listener.accept().await {
                debug!(
                    "connection on {} from {}",
                    port,
                    stream.peer_addr().unwrap()
                );

                match accept_hdr_async(stream, |request: &Request, response: Response| {
                    let their_path = request.uri().path();
                    if their_path != format!("/{}", path) {
                        warn!("invalid path {}", their_path);

                        let response = Response::builder()
                            .status(StatusCode::FORBIDDEN)
                            .body(Some("".into()))
                            .unwrap();
                        Err(response)
                    } else {
                        Ok(response)
                    }
                })
                .await
                {
                    Err(e) => {
                        warn!("{}", e);
                    }

                    Ok(ws_stream) => {
                        spawn_connection(ws_stream).unwrap();
                    }
                }
            }

            let mut guard = CURRENT_LISTEN_FUTURE.lock().unwrap();
            *guard = None;
        }
        .remote_handle();

        async_manager::spawn(f);

        let mut guard = CURRENT_LISTEN_FUTURE.lock().unwrap();
        *guard = Some(remote_handle);
    }

    Ok(ConnectionArgs { port, path })
}

fn spawn_connection(ws_stream: WebSocketStream<TcpStream>) -> Result<()> {
    let (mut write, read) = ws_stream.split();
    let mut read = read.fuse();

    async_manager::spawn(async move {
        let result: Result<()> = async move {
            loop {
                futures::select! {
                    result = read.select_next_some() => {
                        let msg = result?;

                        debug!("{}", msg);

                        if let Ok(text) = msg.into_text() {
                            handle_message(serde_json::from_str(&text)?, &mut write).await?;
                        }
                    }
                };
            }
        }
        .await;

        if let Err(e) = result {
            warn!("{}", e);
        }
    });

    Ok(())
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
#[serde(rename_all = "camelCase")]
enum JsonMessage {
    ChatCommand(String),
}

async fn handle_message<S>(message: JsonMessage, write: &mut S) -> Result<()>
where
    S: Sink<Message> + Unpin,
{
    match message {
        JsonMessage::ChatCommand(text) => {
            chat::send(format!("/{}", text));

            // if write.send(Message::text("ok".to_string())).await.is_err() {
            //     bail!("sending message");
            // }
        }
    }

    Ok(())
}
