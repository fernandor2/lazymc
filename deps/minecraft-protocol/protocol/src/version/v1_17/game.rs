use crate::data::chat::Message;
use crate::{set_packet_id, version::PacketId};
use minecraft_protocol_derive::{Decoder, Encoder};
use nbt::CompoundTag;

set_packet_id!(ServerBoundPluginMessage, 0x0A);
set_packet_id!(ServerBoundKeepAlive, 0x0F);

set_packet_id!(ClientBoundPluginMessage, 0x18);
set_packet_id!(NamedSoundEffect, 0x19);
set_packet_id!(JoinGame, 0x26);
set_packet_id!(PlayerPositionAndLook, 0x38);
set_packet_id!(Respawn, 0x3D);
set_packet_id!(SetTitleSubtitle, 0x57);
set_packet_id!(TimeUpdate, 0x58);
set_packet_id!(SetTitleText, 0x59);
set_packet_id!(SetTitleTimes, 0x5A);
set_packet_id!(ClientBoundKeepAlive, 0x21);

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
pub struct PlayerPositionAndLook {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    #[data_type(with = "var_int")]
    pub teleport_id: i32,
    pub dismount_vehicle: bool,
}

#[derive(Encoder, Decoder, Debug)]
pub struct TimeUpdate {
    pub world_age: i64,
    pub time_of_day: i64,
}

#[derive(Encoder, Decoder, Debug)]
pub struct SetTitleText {
    pub text: Message,
}

#[derive(Encoder, Decoder, Debug)]
pub struct SetTitleSubtitle {
    pub text: Message,
}

#[derive(Encoder, Decoder, Debug)]
pub struct SetTitleTimes {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}

#[derive(Encoder, Decoder, Debug)]
pub struct ClientBoundKeepAlive {
    pub id: u64,
}
