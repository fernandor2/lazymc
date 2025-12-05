use crate::data::chat::Message;
use crate::data::chat::Payload;
use crate::decoder::Decoder;
use crate::encoder::Encoder;
use crate::error::DecodeError;
use crate::impl_json_encoder_decoder;
use crate::version::PacketId;
use crate::{set_packet_id, version};
use minecraft_protocol_derive::{Decoder, Encoder};
use nbt::CompoundTag;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Write};
use uuid::Uuid;

set_packet_id!(JoinGame, 0x29);
set_packet_id!(ClientBoundKeepAlive, 0x24);
set_packet_id!(ChunkData, 0x25);
set_packet_id!(ServerBoundKeepAlive, 0x12);
set_packet_id!(ClientBoundChatMessage, 0x0F);
set_packet_id!(ServerBoundChatMessage, 0x03);

#[derive(Encoder, Decoder, Debug, Clone)]
pub struct JoinGame {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<String>,
    #[data_type(with = "var_int")]
    pub max_players: i32,
    #[data_type(with = "var_int")]
    pub view_distance: i32,
    #[data_type(with = "var_int")]
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    #[data_type(with = "var_int")]
    pub dimension_type: i32,
    pub dimension_name: String,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: u8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: OptionalGlobalPos,
    #[data_type(with = "var_int")]
    pub portal_cooldown: i32,
}

#[derive(Encoder, Decoder, Debug, Clone)]
pub struct GlobalPos {
    pub dimension_name: String,
    pub position: u64,
}

#[derive(Debug, Clone)]
pub struct OptionalGlobalPos(pub Option<GlobalPos>);

impl Decoder for OptionalGlobalPos {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        let has_val = bool::decode(reader)?;
        if has_val {
            Ok(OptionalGlobalPos(Some(GlobalPos::decode(reader)?)))
        } else {
            Ok(OptionalGlobalPos(None))
        }
    }
}

impl Encoder for OptionalGlobalPos {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        match &self.0 {
            Some(val) => {
                true.encode(writer)?;
                val.encode(writer)?;
            }
            None => {
                false.encode(writer)?;
            }
        }
        Ok(())
    }
}

#[derive(Encoder, Decoder, Debug, Clone)]
pub struct ClientBoundKeepAlive {
    pub id: i64,
}

#[derive(Encoder, Decoder, Debug, Clone)]
pub struct ChunkData {
    pub x: i32,
    pub z: i32,
    pub heightmaps: CompoundTag,
    pub data: Vec<u8>,
    pub block_entities: Vec<CompoundTag>,
}

#[derive(Encoder, Decoder, Debug, Clone)]
pub struct ServerBoundKeepAlive {
    pub id: i64,
}

#[derive(Encoder, Decoder, Debug, Clone)]
pub struct ClientBoundChatMessage {
    pub content: String,
}

#[derive(Encoder, Decoder, Debug, Clone)]
pub struct ServerBoundChatMessage {
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}
