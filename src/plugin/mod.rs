use crate::{async_manager, error::Result};

mod command;
mod websocket_server;

pub fn init() {
    async_manager::initialize();

    command::init();
}

pub fn free() {
    command::free();

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
