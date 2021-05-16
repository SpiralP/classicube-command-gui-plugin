use crate::error::Result;
use classicube_helpers::tick::TickEventHandler;
use std::{cell::RefCell, sync::mpsc::channel};
use tracing::debug;

mod command;
mod websocket_server;

thread_local!(
    static TICK_HANDLER: RefCell<Option<TickEventHandler>> = Default::default();
);

pub fn init() {
    command::init();

    let (tx, rx) = channel::<usize>();

    TICK_HANDLER.with(move |cell| {
        let option = &mut *cell.borrow_mut();

        let mut tick_handler = TickEventHandler::new();
        tick_handler.on(move |_| {
            for event in rx.try_iter() {
                debug!("{:#?}", event);
            }
        });

        *option = Some(tick_handler);
    });
}

pub fn free() {
    TICK_HANDLER.with(move |cell| {
        let option = &mut *cell.borrow_mut();
        drop(option.take());
    });

    command::free();
}

pub fn open() -> Result<()> {
    let args = websocket_server::start()?;

    // TODO if dev...
    open::that(format!(
        "http://127.0.0.1:8080/#{}",
        base64::encode(serde_json::to_string(&args)?)
    ))?;

    // https://spiralp.github.io/classicube-command-gui-plugin/

    Ok(())
}
