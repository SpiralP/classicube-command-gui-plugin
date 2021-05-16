use crate::error::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use std::{iter, net::TcpListener, thread::spawn};
use tracing::{debug, warn};
use tungstenite::{
    handshake::server::{Request, Response},
    http::StatusCode,
    server::accept_hdr,
};

#[derive(Serialize)]
pub struct ConnectionArgs {
    pub port: u16,
    pub path: String,
}

#[tracing::instrument]
pub fn start() -> Result<ConnectionArgs> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
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
        spawn(move || {
            for stream_result in listener.incoming() {
                let stream = stream_result.unwrap();
                debug!("connection from {}", stream.peer_addr().unwrap());

                match accept_hdr(stream, |request: &Request, response: Response| {
                    let their_path = request.uri().path();
                    if their_path != format!("/{}", path) {
                        warn!("invalid path {}", their_path);

                        let response = Response::builder()
                            .status(StatusCode::FORBIDDEN)
                            .body(Some("Access denied".into()))
                            .unwrap();
                        Err(response)
                    } else {
                        Ok(response)
                    }
                }) {
                    Err(e) => {
                        warn!("{}", e);
                    }

                    Ok(mut connection) => {
                        spawn(move || {
                            loop {
                                let msg = connection.read_message().unwrap();
                                debug!("{}", msg);

                                // We do not want to send back ping/pong messages.
                                if msg.is_text() {
                                    connection.write_message(msg).unwrap();
                                }
                            }
                        });
                    }
                }
            }
        });
    }

    Ok(ConnectionArgs { port, path })
}
