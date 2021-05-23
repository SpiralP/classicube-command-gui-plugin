pub fn is_ranks_message(message: &str) -> bool {
    (|| {
        Some(
            message.get(0..1)? == "&"
                && message.contains("- Draw: ")
                && message.contains(", Perm: ")
                && message.contains(", max realms: "),
        )
    })()
    .unwrap_or(false)
}

#[test]
fn test_is_ranks_message() {
    assert!(is_ranks_message(
        "&8Banned &7- Draw: 1, Perm: -20, max realms: 3"
    ));
    assert!(is_ranks_message(
        "&7Admin - Draw: 1073741824, Perm: 100, max realms: 64"
    ));
    assert!(!is_ranks_message("naw"));
}
