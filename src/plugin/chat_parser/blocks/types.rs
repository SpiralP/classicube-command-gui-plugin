use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlockProperties {
    pub id: String,
    pub basic: BasicBlockProperties,
    pub complex: ComplexBlockProperties,
}

impl Default for BlockProperties {
    fn default() -> Self {
        Self {
            id: String::new(),
            basic: Default::default(),
            complex: Default::default(),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BasicBlockProperties {
    pub death_message: Option<String>,
    /// Kills players who collide with this block
    pub killer_block: bool,
    /// Is a tdoor (allows other blocks through when open)
    pub is_t_door: bool,
    /// Is an ordinary door
    pub is_door: bool,
    /// Is an odoor (can be toggled by doors, and toggles other odoors)
    pub o_door_block: bool,
    /// Can be used as a &T/MessageBlock
    pub is_message_block: bool,
    /// Can be used as a &T/Portal
    pub is_portal: bool,
    /// Is destroyed by flooding water
    pub water_kills: bool,
    /// Is destroyed by flooding lava
    pub lava_kills: bool,
    /// Is not affected by explosions
    pub op_block: bool,
    /// Can be used as rails for &T/Train
    pub is_rails: bool,
    /// Has the {0} AI behaviour
    pub animal_ai: Option<AnimalAI>,
    /// Stacks as {0} when placed on top of itself
    pub stack_block: Option<String>,
    /// Players can drown in this block
    pub drownable: bool,
    /// Grows into {0} when in sunlight
    pub grass_block: Option<String>,
    /// Decays into {0} when in shadow
    pub dirt_block: Option<String>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AnimalAI {
    None,
    Fly,
    FleeAir,
    KillerAir,
    FleeWater,
    KillerWater,
    FleeLava,
    KillerLava,
}

impl AnimalAI {
    pub fn from_str(s: &str) -> Option<Self> {
        None
    }
}

impl Default for BasicBlockProperties {
    fn default() -> Self {
        Self {
            death_message: None,
            killer_block: false,
            is_t_door: false,
            is_door: false,
            o_door_block: false,
            is_message_block: false,
            is_portal: false,
            water_kills: false,
            lava_kills: false,
            op_block: false,
            is_rails: false,
            animal_ai: None,
            stack_block: None,
            drownable: false,
            grass_block: None,
            dirt_block: None,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComplexBlockProperties {
    /// Appears as a ... block
    pub basic_block_name: Option<String>,
    /// Allows light through
    pub light_pass: bool,
    /// The block's physics will auto-start
    pub need_restart: bool,
    /// Affects physics in some way
    pub physics: bool,
    /// Anybody can activate this block
    pub allow_break: bool,
    /// Can be walked through
    pub walkthrough: bool,
    /// Can be activated by walking through it
    pub walkthrough_activated: bool,
}

impl Default for ComplexBlockProperties {
    fn default() -> Self {
        Self {
            basic_block_name: None,
            light_pass: false,
            need_restart: false,
            physics: false,
            allow_break: false,
            walkthrough: false,
            walkthrough_activated: false,
        }
    }
}
