#[allow(dead_code)]
pub fn is_continuation_message(mut message: &str) -> Option<&str> {
    if message.starts_with("> ") {
        message = message.get(2..)?;
        Some(remove_color_left(message))
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn remove_color_left(mut text: &str) -> &str {
    while text.len() >= 2 {
        if !text.get(0..1).map(|c| c == "&").unwrap_or(false) {
            break;
        }
        if let Some(trimmed) = text.get(2..) {
            text = trimmed;
        } else {
            break;
        }
    }

    text
}

#[test]
fn test_remove_color_left() {
    assert_eq!(remove_color_left("&a&b&cc"), "c");
    assert_eq!(remove_color_left("&&b&cc"), "b&cc");
    assert_eq!(remove_color_left("&&&b&c"), "");
    assert_eq!(remove_color_left("&c"), "");
    assert_eq!(remove_color_left("&"), "&");
}

pub fn remove_color_right(mut text: &str) -> &str {
    while text.len() >= 2 {
        if !text
            .get((text.len() - 2)..(text.len() - 1))
            .map(|c| c == "&")
            .unwrap_or(false)
        {
            break;
        }

        if let Some(trimmed) = text.get(..(text.len() - 2)) {
            text = trimmed;
        } else {
            break;
        }
    }

    text
}

#[test]
fn test_remove_color_right() {
    assert_eq!(remove_color_right("asdf&a&b&c"), "asdf");
    assert_eq!(remove_color_right("asdf&a&b&&c"), "asdf&a&b&");
    assert_eq!(remove_color_right("&&&b&c"), "");
    assert_eq!(remove_color_right("&c"), "");
    assert_eq!(remove_color_right("&"), "&");
}
