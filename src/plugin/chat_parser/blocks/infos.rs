use super::{
    helpers::is_blocks_properties_message,
    types::{AnimalAI, BasicBlockProperties, ComplexBlockProperties},
};
use crate::plugin::chat_parser::helpers::remove_color_left;

pub fn parse_basic_info_message(mut message: &str, p: &mut BasicBlockProperties) -> bool {
    if let Some(m) = is_blocks_properties_message(&message) {
        message = m;
    } else {
        return false;
    }

    if message.starts_with("Death message: ") {
        p.death_message = message
            .get("Death message: ".len()..)
            .map(|a| a.to_string());
        true
    } else if message == "Kills players who collide with this block" {
        p.killer_block = true;
        true
    } else if message == "Is a tdoor (allows other blocks through when open)" {
        p.is_t_door = true;
        true
    } else if message == "Is an ordinary door" {
        p.is_door = true;
        true
    } else if message == "Is an odoor (can be toggled by doors, and toggles other odoors)" {
        p.o_door_block = true;
        true
    } else if message.starts_with("Can be used as a ") && message.ends_with("/MessageBlock") {
        p.is_message_block = true;
        true
    } else if message.starts_with("Can be used as a ") && message.ends_with("/Portal") {
        p.is_portal = true;
        true
    } else if message == "Is destroyed by flooding water" {
        p.water_kills = true;
        true
    } else if message == "Is destroyed by flooding lava" {
        p.lava_kills = true;
        true
    } else if message == "Is not affected by explosions" {
        p.op_block = true;
        true
    } else if message.starts_with("Can be used as rails for ") && message.ends_with("/Train") {
        p.is_rails = true;
        true
    } else if let Some(animal_ai) =
        get_string_between_start_and_end(&message, "Has the ", " AI behaviour")
    {
        p.animal_ai = AnimalAI::from_str(animal_ai);
        true
    } else if let Some(stack_block) =
        get_string_between_start_and_end(&message, "Stacks as ", " when placed on top of itself")
    {
        p.stack_block = Some(stack_block.to_string());
        true
    } else if message == "Players can drown in this block" {
        p.drownable = true;
        true
    } else if let Some(grass_block) =
        get_string_between_start_and_end(&message, "Grows into ", " when in sunlight")
    {
        p.grass_block = Some(grass_block.to_string());
        true
    } else if let Some(dirt_block) =
        get_string_between_start_and_end(&message, "Decays into ", " when in shadow")
    {
        p.dirt_block = Some(dirt_block.to_string());
        true
    } else {
        false
    }
}

pub fn parse_complex_info_message(mut message: &str, p: &mut ComplexBlockProperties) -> bool {
    if let Some(m) = is_blocks_properties_message(&message) {
        message = m;
    } else {
        return false;
    }

    if let Some(basic_block_name) =
        get_string_between_start_and_end(&message, "Appears as a \"", "\" block")
    {
        p.basic_block_name = Some(basic_block_name.to_string());
        true
    } else if remove_color_left(&message) == "Allows light through" {
        p.light_pass = true;
        true
    } else if remove_color_left(&message) == "The block's physics will auto-start" {
        p.need_restart = true;
        true
    } else if remove_color_left(&message) == "Affects physics in some way" {
        p.physics = true;
        true
    } else if remove_color_left(&message) == "Does not affect physics in any way" {
        p.physics = false;
        true
    } else if remove_color_left(&message) == "Anybody can activate this block" {
        p.allow_break = true;
        true
    } else if remove_color_left(&message) == "Can be walked through" {
        p.walkthrough = true;
        true
    } else if remove_color_left(&message) == "Can be activated by walking through it" {
        p.walkthrough_activated = true;
        true
    } else {
        false
    }
}

fn get_string_between_start_and_end<'a>(
    message: &'a str,
    left: &str,
    right: &str,
) -> Option<&'a str> {
    if message.starts_with(left)
        && message.ends_with(right)
        && message.len() >= left.len() + right.len()
    {
        message.get(left.len()..(message.len() - right.len()))
    } else {
        None
    }
}

#[test]
fn test_get_string_between_start_and_end() {
    assert_eq!(
        get_string_between_start_and_end("aaabbbbccc", "aaa", "ccc").unwrap(),
        "bbbb"
    );
    assert_eq!(
        get_string_between_start_and_end("aaaccc", "aaa", "ccc").unwrap(),
        ""
    );
    assert!(get_string_between_start_and_end("aaacc", "aaa", "ccc").is_none());
}
