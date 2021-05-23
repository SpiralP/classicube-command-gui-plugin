mod helpers;

use self::helpers::{is_players_message, is_players_start_message};
use super::{helpers::is_continuation_message, wait_for_message, SHOULD_BLOCK};
use crate::{async_manager, chat, error::*};
use classicube_helpers::CellGetSet;
use std::{collections::HashMap, time::Duration};
use tracing::*;

#[derive(Debug)]
pub struct Rank {
    pub color_code: char,
    pub rank_name: String,
}

#[derive(Debug)]
pub struct Player {
    pub player_name: String,
    pub map_name: String,
}

pub async fn execute() -> Result<HashMap<Rank, Vec<Player>>> {
    async_manager::timeout(Duration::from_secs(3), async {
        chat::send("/Players");

        loop {
            let message = wait_for_message().await;
            if is_players_start_message(&message) {
                SHOULD_BLOCK.set(true);
                break;
            }
            // keep checking other messages until we find ^
        }
    })
    .await
    .chain_err(|| "never found start of /Players response")?;

    let map = read_rank_lines().await?;

    Ok(map)
}

async fn read_rank_lines() -> Result<HashMap<Rank, Vec<Player>>> {
    let mut messages = Vec::new();
    let timeout_result = async_manager::timeout(Duration::from_secs(3), async {
        let mut was_message = false;

        loop {
            let message = wait_for_message().await;

            if was_message {
                if let Some(message) = is_continuation_message(&message) {
                    SHOULD_BLOCK.set(true);

                    let last_message = messages.last_mut().unwrap();
                    *last_message = format!("{} {}", last_message, message);
                    continue;
                }
            }
            if is_players_message(&message) {
                SHOULD_BLOCK.set(true);

                messages.push(message);

                was_message = true;
            } else {
                was_message = false;
            }
        }
    })
    .await;

    if timeout_result.is_none() {
        debug!("stopping because of timeout");
    }

    let map = HashMap::new();

    Ok(map)
}

#[test]
fn test_execute_players() {
    crate::logger::initialize(true, false);
    crate::async_manager::initialize();

    async_manager::spawn_local_on_main_thread(async {
        let map = execute().await.unwrap();
        println!("{:#?}", map);
    });

    // TODO why!
    async_manager::step();

    let messages = vec![
        "&7There are &a10 &7players online.",
        "&7:Admins: goodly (main5)",
        "&7:&oTrusted: mrk (landing)",
        "weird non-Players line",
        "&7:&9Regulars: ┐ Som (main5)",
        "&7:&rArchitects: SpiralP (bridge), ░ Rler (doubletower)",
        "&7:&uMembers: Ranjana (romankalinin1209+)",
    ];

    for message in messages {
        let should_block = super::handle_chat_message(message);
        assert_eq!(
            should_block,
            message != "weird non-Players line",
            "{:?}",
            message
        );
    }

    async_manager::run();
    crate::async_manager::shutdown();
}
