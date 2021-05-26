/*



/Overseer BlockProps list all
&7Grass
&7Showing modified blocks 1-1 (out of 1)

/Blocks 2
&aProperties of Grass:
&7  Is an ordinary door
&7Blocks which look like "Grass":
&uDoor_Grass


/Overseer LevelBlock list all
&7Custom block &a2 &7has name &aStone
&7Showing custom blocks 1-1 (out of 1)

/Overseer LevelBlock info 1
&7About Stone (1)
&7  Draw type: 0, Blocks light: True, collide type: 2
&7  Fallback ID: 1, Sound: 4, Speed: 1.00
&7  Block does not use fog
&7  Block is a cube from (0, 0, 0) to (16, 16, 16)
&7  Texture IDs (left: 1, right: 1, front: 1, back: 1, top: 1,
> &7bottom: 1)
&7  Order: None

*/

mod helpers;
mod infos;
mod types;

pub use self::types::*;
use self::{
    helpers::is_blocks_complex_info_start_message,
    infos::{parse_basic_info_message, parse_complex_info_message},
};
use super::{helpers::is_continuation_message, wait_for_message, CURRENT_COMMAND, SHOULD_BLOCK};
use crate::{
    async_manager, chat,
    error::*,
    plugin::chat_parser::blocks::helpers::{
        is_blocks_looks_like_message, is_blocks_looks_like_none_message,
        is_blocks_looks_like_start_message, is_blocks_start_message,
    },
};
use classicube_helpers::CellGetSet;
use std::time::Duration;

pub async fn execute(block_name: &str) -> Result<BlockProperties> {
    let lock = CURRENT_COMMAND.lock().await;
    let mut properties = BlockProperties::default();

    async_manager::timeout(Duration::from_secs(3), async {
        chat::send(format!("/Blocks {}", block_name));

        loop {
            let message = wait_for_message().await;
            if is_blocks_start_message(&message) {
                SHOULD_BLOCK.set(true);
                break;
            }
        }
    })
    .await
    .chain_err(|| "never found start message for /Blocks response")?;

    let mut has_looks_like = false;
    let mut has_complex_info = false;
    async_manager::timeout(Duration::from_secs(3), async {
        loop {
            let message = wait_for_message().await;
            if parse_basic_info_message(&message, &mut properties.basic) {
                SHOULD_BLOCK.set(true);
            } else if is_blocks_complex_info_start_message(&message) {
                SHOULD_BLOCK.set(true);
                has_complex_info = true;
                break;
            } else if is_blocks_looks_like_start_message(&message) {
                SHOULD_BLOCK.set(true);
                has_looks_like = true;
                break;
            } else if is_blocks_looks_like_none_message(&message) {
                SHOULD_BLOCK.set(true);
                break;
            }
        }
    })
    .await
    .chain_err(|| "never found 1 message after start of /Blocks response")?;

    if has_complex_info {
        let mut found_complex_info_body_message = false;
        let timeout_result = async_manager::timeout(Duration::from_secs(3), async {
            loop {
                let message = wait_for_message().await;
                if parse_complex_info_message(&message, &mut properties.complex) {
                    SHOULD_BLOCK.set(true);
                    found_complex_info_body_message = true;
                }
            }
        })
        .await;

        if !found_complex_info_body_message {
            timeout_result
                .chain_err(|| "never found any 'complex info' messages for /Blocks response")?;
        }
    } else if has_looks_like {
        let mut found_looks_like_message = false;
        let timeout_result = async_manager::timeout(Duration::from_secs(3), async {
            loop {
                let message = wait_for_message().await;
                if is_blocks_looks_like_message(&message) {
                    SHOULD_BLOCK.set(true);
                    found_looks_like_message = true;
                    break;
                }
            }

            loop {
                let message = wait_for_message().await;
                if is_continuation_message(&message).is_some() {
                    SHOULD_BLOCK.set(true);
                } else {
                    break;
                }
            }
        })
        .await;

        if !found_looks_like_message {
            timeout_result
                .chain_err(|| "never found any 'looks like' messages for /Blocks response")?;
        }
    }

    drop(lock);
    Ok(properties)
}

#[test]
fn test_execute_blocks() {
    use self::types::{BasicBlockProperties, ComplexBlockProperties};

    crate::logger::initialize(true, false);

    let mut trials = vec![
        (
            "Air",
            BlockProperties {
                basic: BasicBlockProperties {
                    death_message: Some("@p hit the floor &chard.".to_string()),
                    water_kills: true,
                    lava_kills: true,
                    ..Default::default()
                },
                complex: ComplexBlockProperties {
                    ..Default::default()
                },
            },
            vec![
                "&aProperties of Air:",
                "&7  Death message: @p hit the floor &chard.",
                "&7  Is destroyed by flooding water",
                "&7  Is destroyed by flooding lava",
                "&7Blocks which look like \"Air\":",
                "weird non-Blocks line",
                "&uunknown, unknown, unknown, unknown, unknown, unknown, Op_Air,",
                "> &uunknown, unknown, unknown, unknown, unknown",
            ],
        ),
        (
            "Woodstair-D-N",
            BlockProperties {
                basic: BasicBlockProperties {
                    ..Default::default()
                },
                complex: ComplexBlockProperties {
                    ..Default::default()
                },
            },
            vec![
                "&aProperties of Woodstair-D-N:",
                "&7No complex blocks look like \"Woodstair-D-N\"",
            ],
        ),
        (
            "Door_Lava",
            BlockProperties {
                basic: BasicBlockProperties {
                    is_door: true,
                    ..Default::default()
                },
                complex: ComplexBlockProperties {
                    basic_block_name: Some("Lava".to_string()),
                    physics: false,
                    walkthrough: true,
                    ..Default::default()
                },
            },
            vec![
                "&aProperties of Door_Lava:",
                "&7  Is an ordinary door",
                "weird non-Blocks line",
                "&bComplex information for \"Door_Lava\":",
                "&c  Appears as a \"\" block",
                "&c  Appears as a \"Lava\" block",
                "&c  Appears as a \" block",
                "&7  Does not affect physics in any way",
                "weird non-Blocks line",
                "&7  Can be walked through",
            ],
        ),
    ];

    for (block_name, correct_properties, messages) in trials.drain(..) {
        crate::async_manager::initialize();

        async_manager::spawn_local_on_main_thread(async move {
            let properties = execute(&block_name).await.unwrap();
            assert_eq!(properties, correct_properties);
        });

        // TODO why!
        async_manager::step();

        for message in messages {
            let should_block = super::handle_chat_message(message);
            assert_eq!(
                should_block,
                message != "weird non-Blocks line" && message != "&c  Appears as a \" block",
                "{:?}",
                message
            );
        }

        async_manager::run();
        crate::async_manager::shutdown();
    }
}
