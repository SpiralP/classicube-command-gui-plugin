/*

/Overseer BlockProps list all
&7Grass
&7Showing modified blocks 1-1 (out of 1)

*/

mod helpers;

use self::helpers::is_block_props_message;
use super::{helpers::is_continuation_message, wait_for_message, CURRENT_COMMAND, SHOULD_BLOCK};
use crate::{
    async_manager, chat, error::*,
    plugin::chat_parser::block_props::helpers::is_block_props_end_message,
};
use classicube_helpers::CellGetSet;
use std::time::Duration;
use tracing::*;

pub async fn execute() -> Result<Vec<String>> {
    let lock = CURRENT_COMMAND.lock().await;
    let mut block_names = Vec::new();

    let mut maybe_full_line = async_manager::timeout(Duration::from_secs(3), async {
        chat::send("/Overseer BlockProps list all");

        loop {
            let message = wait_for_message().await;
            if is_block_props_message(&message) {
                SHOULD_BLOCK.set(true);
                return Some(message);
            } else if is_block_props_end_message(&message) {
                SHOULD_BLOCK.set(true);
                return None;
            }
        }
    })
    .await
    .chain_err(|| "never found first message for BlockProps")?;

    if let Some(mut full_line) = maybe_full_line.take() {
        async_manager::timeout(Duration::from_secs(3), async {
            loop {
                let message = wait_for_message().await;
                if let Some(message) = is_continuation_message(&message) {
                    SHOULD_BLOCK.set(true);

                    full_line = format!("{} {}", full_line, message);
                } else {
                    if is_block_props_end_message(&message) {
                        SHOULD_BLOCK.set(true);
                    }
                    break;
                }
            }
        })
        .await
        .chain_err(|| "never found end message for BlockProps response")?;

        if let Some(full_line) = full_line.get(2..) {
            block_names = full_line
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
        }
    }

    drop(lock);
    Ok(block_names)
}

#[test]
fn test_execute_block_props_list() {
    crate::logger::initialize(true, false);

    let mut trials = vec![
        (vec!["&7Showing modified blocks 0-0 (out of 0)"], vec![]),
        (vec!["&7Showing modified blocks 1-0 (out of 0)"], vec![]),
        (
            vec![
                "&7Stone, Grass, Dirt",
                "&7Showing modified blocks 1-3 (out of 3)",
            ],
            vec!["Stone", "Grass", "Dirt"],
        ),
        (
            vec![
                "&7Stone, Grass, Dirt, Cobblebrick, Wood, Sapling, Bedrock,",
                "> &7Activewater",
                "&7Showing modified blocks 1-3 (out of 3)",
            ],
            vec![
                "Stone",
                "Grass",
                "Dirt",
                "Cobblebrick",
                "Wood",
                "Sapling",
                "Bedrock",
                "Activewater",
            ],
        ),
    ];

    for (messages, correct_block_names) in trials.drain(..) {
        crate::async_manager::initialize();

        async_manager::spawn_local_on_main_thread(async move {
            let block_names = execute().await.unwrap();
            assert_eq!(block_names, correct_block_names);
        });

        // TODO why!
        async_manager::step();

        for message in messages {
            let should_block = super::handle_chat_message(message);
            assert_eq!(
                should_block,
                message != "weird non-Blocks line",
                "{:?}",
                message
            );
        }

        async_manager::run();
        crate::async_manager::shutdown();
    }
}
