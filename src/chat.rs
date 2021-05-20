use tracing::*;

pub fn print<S: Into<String>>(s: S) {
    #[allow(unused_mut)]
    let mut s = s.into();
    info!("{}", s);

    #[cfg(not(test))]
    {
        use classicube_sys::{Chat_Add, OwnedString};

        if s.len() > 255 {
            s.truncate(255);
        }

        let owned_string = OwnedString::new(s);

        unsafe {
            Chat_Add(owned_string.as_cc_string());
        }
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
