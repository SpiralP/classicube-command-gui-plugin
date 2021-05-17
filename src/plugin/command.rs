use crate::{async_manager, chat, error::Result, plugin};
use classicube_helpers::color::{RED, WHITE};
use classicube_sys::{cc_string, OwnedChatCommand};
use std::{cell::Cell, os::raw::c_int, slice};

fn chat_print_result(result: Result<()>) {
    if let Err(e) = result {
        chat::print(format!("{}Error: {}{}", RED, WHITE, e));
    }
}

extern "C" fn c_chat_command_callback(args: *const cc_string, args_count: c_int) {
    let args = unsafe { slice::from_raw_parts(args, args_count as _) };
    let args: Vec<String> = args.iter().map(|cc_string| cc_string.to_string()).collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match args.as_slice() {
        ["open"] => {
            async_manager::spawn_local_on_main_thread(async {
                let result =
                    async { Ok(async_manager::spawn(async { plugin::open().await }).await??) };
                chat_print_result(result.await);
            });
        }

        _ => {
            chat::print("/client CommandGui open");
        }
    }
}

thread_local!(
    static COMMAND: Cell<Option<OwnedChatCommand>> = Default::default();
);

pub fn init() {
    COMMAND.with(|cell| {
        let mut command = OwnedChatCommand::new(
            "CommandGui",
            c_chat_command_callback,
            false,
            vec!["/client CommandGui"],
        );

        command.register();

        cell.set(Some(command));
    });
}

pub fn free() {
    COMMAND.with(|cell| {
        drop(cell.replace(None));
    });
}
