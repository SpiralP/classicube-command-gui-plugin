mod helpers;
pub mod ranks;

use crate::{async_manager, chat::IN_CHAT_PRINT};
use classicube_helpers::CellGetSet;
use classicube_sys::{
    MsgType, MsgType_MSG_TYPE_NORMAL, Net_Handler, Protocol, Server, UNSAFE_GetString,
    OPCODE__OPCODE_MESSAGE,
};
use futures::channel::oneshot;
use std::{
    cell::{Cell, RefCell},
    slice,
};

thread_local!(
    pub static SHOULD_BLOCK: Cell<bool> = Cell::new(false);
);

thread_local!(
    static WAITING_FOR_MESSAGE: RefCell<Vec<oneshot::Sender<String>>> = Default::default();
);

thread_local!(
    static OLD_MESSAGE_HANDLER: RefCell<Net_Handler> = Default::default();
);

extern "C" fn message_handler(data: *mut u8) {
    {
        let data = unsafe { slice::from_raw_parts(data, 65) };
        let message_type = data[0] as MsgType;
        let text = unsafe { UNSAFE_GetString(&data[1..]) }.to_string();

        if message_type == MsgType_MSG_TYPE_NORMAL && handle_chat_message(&text) {
            return;
        }
    }

    OLD_MESSAGE_HANDLER.with(|cell| {
        let option = &*cell.borrow();
        let f = option.unwrap();
        unsafe {
            f(data);
        }
    });
}

fn install_message_handler() {
    let old_handler = unsafe { Protocol.Handlers[OPCODE__OPCODE_MESSAGE as usize] };
    unsafe {
        Protocol.Handlers[OPCODE__OPCODE_MESSAGE as usize] = Some(message_handler);
    }

    OLD_MESSAGE_HANDLER.with(|cell| {
        let option = &mut *cell.borrow_mut();
        *option = old_handler;
    });
}

pub fn init() {
    if unsafe { Server.IsSinglePlayer } != 0 {
        return;
    }

    install_message_handler();
}

pub fn reset() {
    init()
}

pub fn free() {}

pub async fn wait_for_message() -> String {
    let (sender, receiver) = oneshot::channel();

    WAITING_FOR_MESSAGE.with(|cell| {
        let waiting = &mut cell.borrow_mut();
        waiting.push(sender);
    });

    // unwrap ok because we don't drop the sender before send()
    receiver.await.unwrap()
}

#[must_use]
pub fn handle_chat_message(message: &str) -> bool {
    // don't recurse from chat::print()
    if IN_CHAT_PRINT.get() {
        return false;
    }

    // resolve the awaits here so that our very next step() will trigger that code section
    let waiting: Vec<_> = WAITING_FOR_MESSAGE.with(|cell| {
        let waiting = &mut cell.borrow_mut();
        waiting.drain(..).collect()
    });

    for sender in waiting {
        let _ignore_error = sender.send(message.to_string());
    }

    async_manager::step();

    // check SHOULD_BLOCK to see if any futures said to block
    let should_block = SHOULD_BLOCK.get();
    SHOULD_BLOCK.set(false);

    should_block
}
