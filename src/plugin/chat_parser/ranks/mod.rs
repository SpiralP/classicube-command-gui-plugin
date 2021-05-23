mod helpers;

use self::helpers::is_ranks_message;
use super::{wait_for_message, SHOULD_BLOCK};
use crate::{async_manager, chat, error::*, plugin::chat_parser::helpers::remove_color_right};
use classicube_helpers::CellGetSet;
use error_chain::bail;
use serde::Serialize;
use std::{collections::HashMap, time::Duration};

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub color_code: char,
    pub rank_name: String,
    pub draw_limit: usize,
    pub permission: i32,
    pub max_realms: usize,
}

pub async fn execute() -> Result<Vec<Rank>> {
    let mut ranks = Vec::new();

    let timeout_result = async_manager::timeout(Duration::from_secs(3), async {
        chat::send("/Ranks");

        loop {
            let message = wait_for_message().await;
            if is_ranks_message(&message) {
                // &8Banned &7- Draw: 1, Perm: -20, max realms: 3
                let mut parts = message.splitn(2, "- ");
                let rank_with_color = remove_color_right(parts.next().chain_err(|| "wtf")?).trim();
                let color_code = rank_with_color.chars().nth(1).chain_err(|| "wtf")?;
                let rank_name = rank_with_color.get(2..).chain_err(|| "wtf")?.to_string();

                let infos = parts.next().chain_err(|| "wtf")?;
                let map = infos
                    .split(", ")
                    .map(|part| {
                        let mut parts = part.splitn(2, ": ");
                        let name = parts.next().chain_err(|| "wtf")?;
                        let value = parts.next().chain_err(|| "wtf")?;
                        Ok((name, value))
                    })
                    .collect::<Result<HashMap<&str, &str>>>()?;

                let draw_limit = map.get("Draw").chain_err(|| "wtf")?.parse()?;
                let permission = map.get("Perm").chain_err(|| "wtf")?.parse()?;
                let max_realms = map.get("max realms").chain_err(|| "wtf")?.parse()?;

                SHOULD_BLOCK.set(true);
                ranks.push(Rank {
                    color_code,
                    rank_name,
                    draw_limit,
                    permission,
                    max_realms,
                });
            }
        }

        #[allow(unreachable_code)]
        Ok::<_, Error>(())
    })
    .await;

    match timeout_result {
        Some(result) => result?,

        None => {
            if ranks.is_empty() {
                bail!("/Ranks query timed out with 0 results!");
            }
        }
    }

    Ok(ranks)
}

#[test]
fn test_execute_ranks() {
    crate::logger::initialize(true, false);
    crate::async_manager::initialize();

    async_manager::spawn_local_on_main_thread(async {
        let ranks = execute().await.unwrap();

        assert!(ranks.contains(&Rank {
            rank_name: "Banned".to_string(),
            color_code: '8',
            draw_limit: 1,
            permission: -20,
            max_realms: 3
        }));
        assert!(ranks.contains(&Rank {
            rank_name: "Admin".to_string(),
            color_code: '7',
            draw_limit: 1073741824,
            permission: 100,
            max_realms: 64
        }));
    });

    // TODO why!
    async_manager::step();

    let messages = vec![
        "&8Banned &7- Draw: 1, Perm: -20, max realms: 3",
        "weird non-Players line",
        "&7Admin - Draw: 1073741824, Perm: 100, max realms: 64",
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
