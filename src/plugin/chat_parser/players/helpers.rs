pub fn is_players_start_message(message: &str) -> bool {
    // &7There are &a10 &7players online.

    (|| {
        Some(
            message.get(0..1)? == "&"
                && message.get(2..)?.starts_with("There are ")
                && message.ends_with("players online."),
        )
    })()
    .unwrap_or(false)
}

#[test]
fn test_is_players_start_message() {
    assert!(is_players_start_message(
        "&7There are &a10 &7players online."
    ));
    assert!(!is_players_start_message(
        "&7:&rArchitects: SpiralP (bridge)"
    ));
    assert!(!is_players_start_message("naw"));
}

pub fn is_players_message(message: &str) -> bool {
    // &7:&rArchitects: SpiralP (bridge), ░ Rler (doubletower)
    (|| {
        Some(
            message.get(0..1)? == "&"
                && message.get(2..3)? == ":"
                && message.get(5..)?.contains(":"),
        )
    })()
    .unwrap_or(false)
}

#[test]
fn test_is_players_message() {
    assert!(!is_players_message("&7There are &a10 &7players online."));
    assert!(is_players_message("&7:&rArchitects: SpiralP (bridge)"));
    assert!(!is_players_message("naw"));
}
