pub fn is_block_props_message(message: &str) -> bool {
    (|| {
        Some(
            message.get(0..1)? == "&" && message.matches('&').count() == 1 && {
                let mut parts = message.split(' ').collect::<Vec<_>>();
                parts.pop();
                parts.iter().all(|s| s.ends_with(','))
            },
        )
    })()
    .unwrap_or(false)
}

#[test]
fn test_is_block_props_message() {
    assert!(is_block_props_message("&7Grass, And-Friends"));
    assert!(is_block_props_message("&7Grass, And-Friends,"));
    assert!(is_block_props_message("&7Grass"));
    assert!(!is_block_props_message("&7Grass and friends"));
    assert!(!is_block_props_message("&7Gras&7s"));
}

pub fn is_block_props_end_message(message: &str) -> bool {
    (|| {
        Some(message.get(0..1)? == "&" && message.get(2..)?.starts_with("Showing modified blocks "))
    })()
    .unwrap_or(false)
}

#[test]
fn test_is_block_props_end_message() {
    assert!(is_block_props_end_message(
        "&7Showing modified blocks 0-0 (out of 0)"
    ));
    assert!(is_block_props_end_message(
        "&7Showing modified blocks 1-0 (out of 0)"
    ));
    assert!(is_block_props_end_message(
        "&7Showing modified blocks 1-2 (out of 2)"
    ));
}
