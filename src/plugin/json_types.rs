use super::{
    chat_parser::{blocks::BlockProperties, ranks::Rank},
    helpers::{bitmap_col_a, bitmap_col_b, bitmap_col_g, bitmap_col_r},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonPlayer {
    pub id: u8,
    pub real_name: String,
    pub nick_name: String,
    pub group: String,
    pub rank: u8,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
#[serde(rename_all = "camelCase")]
pub enum JsonEvent {
    NewPlayers(Vec<JsonPlayer>),
    PlayerAdded(JsonPlayer),
    PlayerRemoved {
        id: u8,
    },
    PlayerChanged(JsonPlayer),
    WeDisconnected,
    ColorCodes(Vec<ColorCode>),
    RenderedText {
        text: String,
        size: u8,
        shadow: bool,
        // R G B A order
        pixels: Vec<u8>,
        width: usize,
        height: usize,
    },
    Ranks(Vec<Rank>),
    Blocks(Vec<JsonBlock>),
    BlockProperties(BlockProperties),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorCode {
    pub char: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
#[serde(rename_all = "camelCase")]
pub enum JsonMessage {
    ChatCommand(String),
    TabListSubscribe,
    AskColorCodes,
    AskRanks,
    AskBlocks,
    AskBlockProperties(String),
    RenderText {
        text: String,
        size: u8,
        shadow: bool,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonBlock {
    id: usize,
    name: String,
    is_liquid: bool,
    blocks_light: bool,
    full_bright: bool,
    fog_col: JsonColor,
    fog_density: f32,
    collide: u8,
    extended_collide: u8,
    speed_multiplier: f32,
    light_offset: u8,
    draw: u8,
    dig_sounds: u8,
    step_sounds: u8,
    tinted: bool,
    full_opaque: bool,
    sprite_offset: u8,
    min_bb: JsonVec3,
    max_bb: JsonVec3,
    render_min_bb: JsonVec3,
    render_max_bb: JsonVec3,
    /* Texture ids of each face of blocks. */
    // textures: [TextureLoc; 4608],
    can_place: bool,
    can_delete: bool,
    /* Bit flags of faces hidden of two neighbouring blocks. */
    // hidden: [u8; 589824],
    can_stretch: u8,
}
impl JsonBlock {
    pub unsafe fn from_id(id: usize) -> Self {
        use classicube_sys::{Block_UNSAFE_GetName, Blocks};

        let name = Block_UNSAFE_GetName(id as _).to_string();

        Self {
            id,
            name,
            is_liquid: Blocks.IsLiquid[id] != 0,
            blocks_light: Blocks.BlocksLight[id] != 0,
            full_bright: Blocks.FullBright[id] != 0,
            fog_col: Blocks.FogCol[id].into(),
            fog_density: Blocks.FogDensity[id],
            collide: Blocks.Collide[id],
            extended_collide: Blocks.ExtendedCollide[id],
            speed_multiplier: Blocks.SpeedMultiplier[id],
            light_offset: Blocks.LightOffset[id],
            draw: Blocks.Draw[id],
            dig_sounds: Blocks.DigSounds[id],
            step_sounds: Blocks.StepSounds[id],
            tinted: Blocks.Tinted[id] != 0,
            full_opaque: Blocks.FullOpaque[id] != 0,
            sprite_offset: Blocks.SpriteOffset[id],
            min_bb: Blocks.MinBB[id].into(),
            max_bb: Blocks.MaxBB[id].into(),
            render_min_bb: Blocks.RenderMinBB[id].into(),
            render_max_bb: Blocks.RenderMaxBB[id].into(),
            // textures: [TextureLoc; 4608],
            can_place: Blocks.CanPlace[id] != 0,
            can_delete: Blocks.CanDelete[id] != 0,
            // hidden: [u8; 589824],
            can_stretch: Blocks.CanStretch[id],
        }
    }

    pub unsafe fn get_all() -> Vec<Self> {
        use classicube_sys::{BLOCKID_BLOCK_AIR, BLOCKID_BLOCK_COUNT};

        (BLOCKID_BLOCK_AIR as usize..BLOCKID_BLOCK_COUNT as usize)
            .map(|id| Self::from_id(id))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonVec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl From<classicube_sys::Vec3> for JsonVec3 {
    fn from(other: classicube_sys::Vec3) -> Self {
        Self {
            x: other.X,
            y: other.Y,
            z: other.Z,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl From<classicube_sys::PackedCol> for JsonColor {
    fn from(other: classicube_sys::PackedCol) -> Self {
        Self {
            r: bitmap_col_r(other),
            g: bitmap_col_g(other),
            b: bitmap_col_b(other),
            a: bitmap_col_a(other),
        }
    }
}
