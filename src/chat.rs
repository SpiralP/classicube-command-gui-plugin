use std::cell::Cell;
use tracing::*;

thread_local!(
    pub static IN_CHAT_PRINT: Cell<bool> = Cell::new(false);
);

pub fn print<S: Into<String>>(s: S) {
    #[allow(unused_mut)]
    let mut s = s.into();
    info!("{}", s);

    #[cfg(not(test))]
    {
        use classicube_helpers::CellGetSet;
        use classicube_sys::{Chat_Add, OwnedString};

        if s.len() > 255 {
            s.truncate(255);
        }

        let owned_string = OwnedString::new(s);

        IN_CHAT_PRINT.set(true);
        unsafe {
            Chat_Add(owned_string.as_cc_string());
        }
        IN_CHAT_PRINT.set(false);
    }
}

pub fn send<S: Into<String>>(s: S) {
    let s = s.into();
    info!("{}", s);

    #[cfg(not(test))]
    {
        use classicube_sys::{Chat_Send, OwnedString};

        let owned_string = OwnedString::new(s);

        unsafe {
            Chat_Send(owned_string.as_cc_string(), 0);
        }
    }
}
