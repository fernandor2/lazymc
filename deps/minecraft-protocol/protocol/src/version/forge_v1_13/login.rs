use crate::{set_packet_id, version::PacketId};
use minecraft_protocol_derive::Decoder;
use minecraft_protocol_derive::Encoder;

set_packet_id!(ModList, 1);
set_packet_id!(ModListReply, 2);
set_packet_id!(ServerRegistry, 3);
set_packet_id!(ConfigurationData, 4);
set_packet_id!(Acknowledgement, 99);

#[derive(Encoder, Decoder, Debug)]
pub struct LoginWrapper {
    pub channel: String,
    pub packet: Vec<u8>,
}

#[derive(Encoder, Decoder, Debug)]
pub struct ModList {
    pub mod_names: Vec<String>,
    pub channels: Vec<(String, String)>,
    pub registries: Vec<String>,
}

impl ModList {
    /// Transform this into a `ModListReply` with empty registry markers.
    pub fn into_reply(self) -> ModListReply {
        ModListReply {
            mod_names: self.mod_names,
            channels: self.channels,
            registries: self
                .registries
                .into_iter()
                .map(|r| (r, "".into()))
                .collect(),
        }
    }
}

#[derive(Encoder, Decoder, Debug)]
pub struct ModListReply {
    pub mod_names: Vec<String>,
    pub channels: Vec<(String, String)>,
    pub registries: Vec<(String, String)>,
}

#[derive(Encoder, Decoder, Debug)]
pub struct ServerRegistry {
    pub registry_name: String,
    pub snapshot_present: bool,
    // TODO: implement this snapshot type!
    // TODO: add dummy data here, 5x 0 ?
    pub snapshot: Vec<u8>,
}

#[derive(Encoder, Decoder, Debug)]
pub struct ConfigurationData {
    pub file_name: String,
    pub data: Vec<u8>,
}

#[derive(Encoder, Decoder, Debug)]
pub struct Acknowledgement {}
