pub fn is_blocks_start_message(message: &str, block_name: &str) -> bool {
    message == format!("&aProperties of {}:", block_name)
}

#[test]
fn test_is_blocks_start_message() {
    assert!(is_blocks_start_message("&aProperties of Air:", "Air"));
    assert!(is_blocks_start_message(
        "&aProperties of Woodstair-D-N:",
        "Woodstair-D-N"
    ));
    assert!(!is_blocks_start_message("naw", "naw"));
}

pub fn is_blocks_properties_message(message: &str) -> Option<&str> {
    if message.get(0..1)? == "&" && message.get(2..4)? == "  " {
        Some(message.get(4..)?)
    } else {
        None
    }
}

#[test]
fn test_is_blocks_properties_message() {
    assert_eq!(
        is_blocks_properties_message("&7  Death message: @p hit the floor &chard.").unwrap(),
        "Death message: @p hit the floor &chard."
    );
    assert!(is_blocks_properties_message("&7  Is destroyed by flooding water").is_some());
    assert!(is_blocks_properties_message("&7  Is destroyed by flooding lava").is_some());
    assert!(is_blocks_properties_message("naw").is_none());
}

pub fn is_blocks_complex_info_start_message(message: &str, block_name: &str) -> bool {
    message == format!("&bComplex information for \"{}\":", block_name)
}

#[test]
fn test_is_blocks_complex_info_start_message() {
    assert!(is_blocks_complex_info_start_message(
        "&bComplex information for \"Door_Lava\":",
        "Door_Lava"
    ));
    assert!(!is_blocks_complex_info_start_message("naw", "naw"));
}

pub fn is_blocks_looks_like_start_message(message: &str, block_name: &str) -> bool {
    message == format!("&7Blocks which look like \"{}\":", block_name)
}

#[test]
fn test_is_blocks_looks_like_start_message() {
    assert!(is_blocks_looks_like_start_message(
        "&7Blocks which look like \"Air\":",
        "Air"
    ));
    assert!(is_blocks_looks_like_start_message(
        "&7Blocks which look like \"Woodstair-D-N\":",
        "Woodstair-D-N"
    ));
    assert!(!is_blocks_looks_like_start_message("naw", "naw"));
}

pub fn is_blocks_looks_like_message(message: &str) -> bool {
    // TODO very generic
    (|| Some(message.get(0..2)? == "&u"))().unwrap_or(false)
}

#[test]
fn test_is_blocks_looks_like_message() {
    assert!(is_blocks_looks_like_message(
        "&uunknown, unknown, unknown, unknown, unknown, unknown, Op_Air,"
    ));
    assert!(is_blocks_looks_like_message(
        "&uOp_Stone, Door_Stone, tDoor_Stone, oDoor_Stone"
    ));
    assert!(!is_blocks_looks_like_message("naw"));
}

pub fn is_blocks_looks_like_none_message(message: &str, block_name: &str) -> bool {
    message == format!("&7No complex blocks look like \"{}\"", block_name)
}

#[test]
fn test_is_blocks_looks_like_none_message() {
    assert!(is_blocks_looks_like_none_message(
        "&7No complex blocks look like \"Woodstair-D-N\"",
        "Woodstair-D-N"
    ));
    assert!(!is_blocks_looks_like_none_message("naw", "naw"));
}
