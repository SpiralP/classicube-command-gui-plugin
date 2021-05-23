#![feature(const_fn_trait_bound)]

mod async_manager;
mod chat;
mod error;
mod logger;
mod plugin;

use classicube_sys::IGameComponent;
use std::{os::raw::c_int, ptr};
use tracing::debug;

extern "C" fn init() {
    logger::initialize(true, false);

    tracing::debug_span!("init").in_scope(|| {
        debug!(
            "Init {}",
            concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"))
        );

        plugin::init();
    })
}

#[tracing::instrument]
extern "C" fn free() {
    debug!("Free");

    plugin::free();
}

#[tracing::instrument]
extern "C" fn reset() {
    debug!("Reset");

    plugin::reset();
}

#[tracing::instrument]
extern "C" fn on_new_map() {}

#[tracing::instrument]
extern "C" fn on_new_map_loaded() {
    use std::sync::Once;

    static START: Once = Once::new();

    START.call_once(|| {
        // run initialization here
        async_manager::spawn(async {
            plugin::open().await.unwrap();
        });
    });
}

#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static mut Plugin_Component: IGameComponent = IGameComponent {
    // Called when the game is being loaded.
    Init: Some(init),
    // Called when the component is being freed. (e.g. due to game being closed)
    Free: Some(free),
    // Called to reset the component's state. (e.g. reconnecting to server)
    Reset: Some(reset),
    // Called to update the component's state when the user begins loading a new map.
    OnNewMap: Some(on_new_map),
    // Called to update the component's state when the user has finished loading a new map.
    OnNewMapLoaded: Some(on_new_map_loaded),
    // Next component in linked list of components.
    next: ptr::null_mut(),
};
