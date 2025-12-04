use crate::data::chat::Message;
use crate::error::DecodeError;
use crate::{set_packet_id, version::PacketId};
use byteorder::{ReadBytesExt, WriteBytesExt};
use minecraft_protocol_derive::{Decoder, Encoder};
use nbt::CompoundTag;

set_packet_id!(ServerBoundPluginMessage, 0x0B);
set_packet_id!(ServerBoundKeepAlive, 0x10);

set_packet_id!(ClientBoundPluginMessage, 0x17);
set_packet_id!(NamedSoundEffect, 0x18);
set_packet_id!(JoinGame, 0x24);
set_packet_id!(PlayerPositionAndLook, 0x34);
set_packet_id!(Respawn, 0x39);
set_packet_id!(TimeUpdate, 0x4E);
set_packet_id!(Title, 0x4F);
set_packet_id!(ClientBoundKeepAlive, 0x1F);

#[derive(Encoder, Decoder, Debug)]
pub struct ServerBoundPluginMessage {
    #[data_type(max_length = 32767)]
    pub channel: String,
    pub data: Vec<u8>,
}

#[derive(Encoder, Decoder)]
pub struct ServerBoundKeepAlive {
    pub id: u64,
}

#[derive(Encoder, Decoder, Debug)]
pub struct ClientBoundPluginMessage {
    #[data_type(max_length = 32767)]
    pub channel: String,
    pub data: Vec<u8>,
}

// TODO(timvisee): remove clone?
#[derive(Clone, Encoder, Decoder, Debug)]
pub struct NamedSoundEffect {
    #[data_type(max_length = 32767)]
    pub sound_name: String,
    #[data_type(with = "var_int")]
    pub sound_category: i32,
    // Mulitplied by 8
    pub effect_pos_x: i32,
    // Mulitplied by 8
    pub effect_pos_y: i32,
    // Mulitplied by 8
    pub effect_pos_z: i32,
    pub volume: f32,
    pub pitch: f32,
}

// TODO(timvisee): remove clone?
#[derive(Clone, Encoder, Decoder, Debug)]
pub struct JoinGame {
    pub entity_id: u32,
    pub hardcore: bool,
    pub game_mode: u8,
    pub previous_game_mode: u8,
    // TODO: max string length: 32767
    pub world_names: Vec<String>,
    pub dimension_codec: CompoundTag,
    pub dimension: CompoundTag,
    #[data_type(max_length = 32767)]
    pub world_name: String,
    pub hashed_seed: i64,
    #[data_type(with = "var_int")]
    pub max_players: i32,
    #[data_type(with = "var_int")]
    pub view_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}

#[derive(Encoder, Decoder, Debug)]
pub struct PlayerPositionAndLook {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    #[data_type(with = "var_int")]
    pub teleport_id: i32,
}

#[derive(Encoder, Decoder, Debug)]
pub struct Respawn {
    pub dimension: CompoundTag,
    #[data_type(max_length = 32767)]
    pub world_name: String,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: u8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub copy_metadata: bool,
}

#[derive(Encoder, Decoder, Debug)]
pub struct TimeUpdate {
    pub world_age: i64,
    pub time_of_day: i64,
}

#[derive(Encoder, Decoder, Debug)]
pub struct Title {
    pub action: TitleAction,
}

#[derive(Encoder, Decoder, Debug)]
pub enum TitleAction {
    SetTitle {
        text: Message,
    },
    SetSubtitle {
        text: Message,
    },
    SetActionBar {
        text: Message,
    },
    SetTimesAndDisplay {
        fade_in: i32,
        stay: i32,
        fade_out: i32,
    },
    Hide,
    Reset,
}

#[derive(Encoder, Decoder, Debug)]
pub struct ClientBoundKeepAlive {
    pub id: u64,
}
