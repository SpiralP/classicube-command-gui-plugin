use crate::{async_manager, error::Result};

mod chat_parser;
mod command;
mod helpers;
mod json_types;
mod map_url_listener;
mod render_text;
mod tab_list_events;
mod websocket_server;

pub fn init() {
    async_manager::initialize();

    map_url_listener::init();
    tab_list_events::init();
    chat_parser::init();

    command::init();
}

pub fn reset() {
    chat_parser::reset();
}

pub fn free() {
    command::free();

    chat_parser::free();
    tab_list_events::free();
    map_url_listener::free();

    // this will stop all tasks immediately
    async_manager::shutdown();
}

pub async fn open() -> Result<()> {
    let args = websocket_server::start().await?;

    // TODO if dev...
    open::that(format!(
        "http://127.0.0.1:8080/#{}",
        base64::encode(serde_json::to_string(&args)?)
    ))?;

    // https://spiralp.github.io/classicube-command-gui-plugin/

    Ok(())
}
