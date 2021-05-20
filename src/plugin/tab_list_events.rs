use super::json_types::{JsonEvent, JsonPlayer};
use crate::async_manager;
use classicube_helpers::{events::tab_list, tab_list::TabList};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Mutex};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

lazy_static! {
    static ref TAB_LIST_EVENT: Mutex<(broadcast::Sender<JsonEvent>, broadcast::Receiver<JsonEvent>)> =
        Mutex::new(broadcast::channel(256));
}

thread_local!(
    static TAB_LIST: RefCell<Option<TabList>> = RefCell::new(None);
);

pub fn make_new_listener() -> BroadcastStream<JsonEvent> {
    let guard = TAB_LIST_EVENT.lock().unwrap();
    BroadcastStream::new(guard.0.subscribe())
}

pub async fn get_current_players() -> Vec<JsonPlayer> {
    async_manager::run_on_main_thread(async move {
        TAB_LIST.with(|cell| {
            let global = &mut *cell.borrow_mut();
            global
                .as_ref()
                .expect("TAB_LIST unset")
                .get_all()
                .values()
                .map(|entry| JsonPlayer {
                    id: entry.get_id(),
                    real_name: entry.get_real_name().unwrap(),
                    nick_name: entry.get_nick_name().unwrap(),
                    group: entry.get_group().unwrap(),
                    rank: entry.get_rank().unwrap(),
                })
                .collect::<Vec<_>>()
        })
    })
    .await
}

pub fn init() {
    TAB_LIST.with(|cell| {
        let global = &mut *cell.borrow_mut();

        let mut tab_list = TabList::new();

        tab_list.on_added(move |tab_list::AddedEvent { entry }| {
            let guard = TAB_LIST_EVENT.lock().unwrap();
            guard
                .0
                .send(JsonEvent::PlayerAdded(JsonPlayer {
                    id: entry.get_id(),
                    real_name: entry.get_real_name().unwrap(),
                    nick_name: entry.get_nick_name().unwrap(),
                    group: entry.get_group().unwrap(),
                    rank: entry.get_rank().unwrap(),
                }))
                .unwrap();
        });

        tab_list.on_removed(move |tab_list::RemovedEvent { id }| {
            let guard = TAB_LIST_EVENT.lock().unwrap();
            guard.0.send(JsonEvent::PlayerRemoved { id: *id }).unwrap();
        });

        tab_list.on_changed(move |tab_list::ChangedEvent { entry }| {
            let guard = TAB_LIST_EVENT.lock().unwrap();
            guard
                .0
                .send(JsonEvent::PlayerChanged(JsonPlayer {
                    id: entry.get_id(),
                    real_name: entry.get_real_name().unwrap(),
                    nick_name: entry.get_nick_name().unwrap(),
                    group: entry.get_group().unwrap(),
                    rank: entry.get_rank().unwrap(),
                }))
                .unwrap();
        });

        tab_list.on_disconnected(move |_| {
            let guard = TAB_LIST_EVENT.lock().unwrap();
            guard.0.send(JsonEvent::WeDisconnected).unwrap();
        });

        *global = Some(tab_list);
    });
}

pub fn free() {}
