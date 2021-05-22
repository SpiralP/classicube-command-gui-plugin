use super::{
    helpers::BitmapCol_A,
    json_types::{JsonEvent, JsonMessage},
    tab_list_events,
};
use crate::{
    async_manager, chat,
    error::*,
    plugin::{
        helpers::{make_text_bitmap, BitmapCol_B, BitmapCol_G, BitmapCol_R},
        json_types::ColorCode,
    },
};
use classicube_sys::Drawer2D;
use futures::{future::RemoteHandle, stream::SplitSink, FutureExt, SinkExt, StreamExt};
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
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

fn make_message(event: JsonEvent) -> Result<Message> {
    Ok(Message::text(serde_json::to_string(&event)?))
}

fn spawn_connection(ws_stream: WebSocketStream<TcpStream>) -> Result<()> {
    let (mut connection_tx, connection_rx) = ws_stream.split();
    let mut connection_rx = connection_rx.fuse();

    async_manager::spawn(async move {
        let result: Result<()> = async move {
            let mut player_event_subscribed = false;
            let mut player_event_listener = tab_list_events::make_new_listener().fuse();

            loop {
                futures::select! {
                    result = connection_rx.select_next_some() => {
                        let msg = result?;

                        debug!("{}", msg);

                        handle_incoming(msg, &mut player_event_subscribed, &mut connection_tx)
                            .await?;
                    }

                    result = player_event_listener.select_next_some() => {
                        if player_event_subscribed {
                            let event = result.chain_err(|| "BroadcastStreamRecvError")?;
                            debug!("{:#?}", event);

                            connection_tx
                                .send(make_message(event)?)
                                .await
                                .chain_err(|| "sending message")?;
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

async fn handle_incoming(
    msg: Message,
    player_event_subscribed: &mut bool,
    connection_tx: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
) -> Result<()> {
    if let Ok(text) = msg.into_text() {
        match serde_json::from_str(&text)? {
            JsonMessage::ChatCommand(text) => {
                async_manager::spawn_on_main_thread(async move {
                    chat::send(format!("/{}", text));
                });
            }

            JsonMessage::TabListSubscribe => {
                let current_players = tab_list_events::get_current_players().await;
                *player_event_subscribed = true;

                connection_tx
                    .send(make_message(JsonEvent::NewPlayers(current_players))?)
                    .await
                    .chain_err(|| "sending message")?;
            }

            JsonMessage::AskColorCodes => {
                let codes = (0..=255u8)
                    .filter_map(|i| {
                        let n = unsafe { Drawer2D.Colors[i as usize] };
                        if BitmapCol_A(n) != 0 {
                            let r: u8 = BitmapCol_R(n);
                            let g: u8 = BitmapCol_G(n);
                            let b: u8 = BitmapCol_B(n);
                            Some(ColorCode {
                                char: (i as char).to_string(),
                                color: format!("{:02X}{:02X}{:02X}", r, g, b),
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                connection_tx
                    .send(make_message(JsonEvent::ColorCodes(codes))?)
                    .await
                    .chain_err(|| "sending message")?;
            }

            JsonMessage::RenderText(text) => {
                let (pixels, width, height) = make_text_bitmap(&text)?;

                connection_tx
                    .send(make_message(JsonEvent::RenderedText {
                        text,
                        pixels,
                        width,
                        height,
                    })?)
                    .await
                    .chain_err(|| "sending message")?;
            }
        }
    }

    Ok(())
}

/*

my name
* title, color

all players names
* tp

players on my map
* slap, os kick

my rank
* which commands i can execute (need smarter checking for extra permission commands)

all players ranks?

*/
