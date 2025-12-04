use minecraft_protocol::decoder::Decoder;
use minecraft_protocol::error::DecodeError;
use minecraft_protocol::version::{v1_16_3, v1_17, v1_20_5, v1_21, v1_21_2, v1_21_4};
use nbt::CompoundTag;
#[cfg(feature = "lobby")]
use tokio::net::tcp::WriteHalf;

#[cfg(feature = "lobby")]
use crate::mc::dimension;
#[cfg(feature = "lobby")]
use crate::proto::client::Client;
use crate::proto::client::ClientInfo;
#[cfg(feature = "lobby")]
use crate::proto::packet;
use crate::proto::packet::RawPacket;
#[cfg(feature = "lobby")]
use crate::server::Server;

/// Data extracted from `JoinGame` packet.
#[derive(Debug, Clone)]
pub struct JoinGameData {
    pub hardcore: Option<bool>,
    pub game_mode: Option<u8>,
    pub previous_game_mode: Option<u8>,
    pub world_names: Option<Vec<String>>,
    pub dimension: Option<CompoundTag>,
    pub dimension_codec: Option<CompoundTag>,
    pub world_name: Option<String>,
    pub hashed_seed: Option<i64>,
    pub max_players: Option<i32>,
    pub view_distance: Option<i32>,
    pub reduced_debug_info: Option<bool>,
    pub enable_respawn_screen: Option<bool>,
    pub is_debug: Option<bool>,
    pub is_flat: Option<bool>,
    // New fields
    pub simulation_distance: Option<i32>,
    pub do_limited_crafting: Option<bool>,
    pub dimension_type: Option<i32>,
    pub dimension_name: Option<String>,
    pub portal_cooldown: Option<i32>,
}

impl JoinGameData {
    /// Extract join game data from given packet.
    pub fn from_packet(client_info: &ClientInfo, packet: RawPacket) -> Result<Self, DecodeError> {
        // Minecraft 1.20.2 is Protocol 764.
        match client_info.protocol() {
            Some(p) if p < v1_17::PROTOCOL => {
                Ok(v1_16_3::game::JoinGame::decode(&mut packet.data.as_slice())?.into())
            }
            Some(p) if p < 764 => { // 1.20.2
                Ok(v1_17::game::JoinGame::decode(&mut packet.data.as_slice())?.into())
            }
            Some(p) if p < v1_21::PROTOCOL => {
                // 1.20.2 (764), 1.20.3/4 (765), 1.20.5/6 (766)
                Ok(v1_20_5::game::JoinGame::decode(&mut packet.data.as_slice())?.into())
            }
            Some(p) if p < v1_21_2::PROTOCOL => {
                Ok(v1_21::game::JoinGame::decode(&mut packet.data.as_slice())?.into())
            }
            Some(p) if p < v1_21_4::PROTOCOL => {
                Ok(v1_21_2::game::JoinGame::decode(&mut packet.data.as_slice())?.into())
            }
            _ => Ok(v1_21_4::game::JoinGame::decode(&mut packet.data.as_slice())?.into()),
        }
    }
}

impl From<v1_16_3::game::JoinGame> for JoinGameData {
    fn from(join_game: v1_16_3::game::JoinGame) -> Self {
        Self {
            hardcore: Some(join_game.hardcore),
            game_mode: Some(join_game.game_mode),
            previous_game_mode: Some(join_game.previous_game_mode),
            world_names: Some(join_game.world_names.clone()),
            dimension: Some(join_game.dimension),
            dimension_codec: Some(join_game.dimension_codec),
            world_name: Some(join_game.world_name),
            hashed_seed: Some(join_game.hashed_seed),
            max_players: Some(join_game.max_players),
            view_distance: Some(join_game.view_distance),
            reduced_debug_info: Some(join_game.reduced_debug_info),
            enable_respawn_screen: Some(join_game.enable_respawn_screen),
            is_debug: Some(join_game.is_debug),
            is_flat: Some(join_game.is_flat),
            simulation_distance: None,
            do_limited_crafting: None,
            dimension_type: None,
            dimension_name: None,
            portal_cooldown: None,
        }
    }
}

impl From<v1_17::game::JoinGame> for JoinGameData {
    fn from(join_game: v1_17::game::JoinGame) -> Self {
        Self {
            hardcore: Some(join_game.hardcore),
            game_mode: Some(join_game.game_mode),
            previous_game_mode: Some(join_game.previous_game_mode),
            world_names: Some(join_game.world_names.clone()),
            dimension: Some(join_game.dimension),
            dimension_codec: Some(join_game.dimension_codec),
            world_name: Some(join_game.world_name),
            hashed_seed: Some(join_game.hashed_seed),
            max_players: Some(join_game.max_players),
            view_distance: Some(join_game.view_distance),
            reduced_debug_info: Some(join_game.reduced_debug_info),
            enable_respawn_screen: Some(join_game.enable_respawn_screen),
            is_debug: Some(join_game.is_debug),
            is_flat: Some(join_game.is_flat),
            simulation_distance: None,
            do_limited_crafting: None,
            dimension_type: None,
            dimension_name: None,
            portal_cooldown: None,
        }
    }
}

impl From<v1_20_5::game::JoinGame> for JoinGameData {
    fn from(join_game: v1_20_5::game::JoinGame) -> Self {
        Self {
            hardcore: Some(join_game.is_hardcore),
            game_mode: Some(join_game.game_mode),
            previous_game_mode: Some(join_game.previous_game_mode),
            world_names: Some(join_game.dimension_names.clone()),
            dimension: None, // Missing
            dimension_codec: None, // Missing
            world_name: Some(join_game.dimension_name.clone()),
            hashed_seed: Some(join_game.hashed_seed),
            max_players: Some(join_game.max_players),
            view_distance: Some(join_game.view_distance),
            reduced_debug_info: Some(join_game.reduced_debug_info),
            enable_respawn_screen: Some(join_game.enable_respawn_screen),
            is_debug: Some(join_game.is_debug),
            is_flat: Some(join_game.is_flat),
            simulation_distance: Some(join_game.simulation_distance),
            do_limited_crafting: Some(join_game.do_limited_crafting),
            dimension_type: Some(join_game.dimension_type),
            dimension_name: Some(join_game.dimension_name),
            portal_cooldown: Some(join_game.portal_cooldown),
        }
    }
}

impl From<v1_21::game::JoinGame> for JoinGameData {
    fn from(join_game: v1_21::game::JoinGame) -> Self {
        Self {
            hardcore: Some(join_game.is_hardcore),
            game_mode: Some(join_game.game_mode),
            previous_game_mode: Some(join_game.previous_game_mode),
            world_names: Some(join_game.dimension_names.clone()),
            dimension: None,
            dimension_codec: None,
            world_name: Some(join_game.dimension_name.clone()),
            hashed_seed: Some(join_game.hashed_seed),
            max_players: Some(join_game.max_players),
            view_distance: Some(join_game.view_distance),
            reduced_debug_info: Some(join_game.reduced_debug_info),
            enable_respawn_screen: Some(join_game.enable_respawn_screen),
            is_debug: Some(join_game.is_debug),
            is_flat: Some(join_game.is_flat),
            simulation_distance: Some(join_game.simulation_distance),
            do_limited_crafting: Some(join_game.do_limited_crafting),
            dimension_type: Some(join_game.dimension_type),
            dimension_name: Some(join_game.dimension_name),
            portal_cooldown: Some(join_game.portal_cooldown),
        }
    }
}

impl From<v1_21_2::game::JoinGame> for JoinGameData {
    fn from(join_game: v1_21_2::game::JoinGame) -> Self {
        Self {
            hardcore: Some(join_game.is_hardcore),
            game_mode: Some(join_game.game_mode),
            previous_game_mode: Some(join_game.previous_game_mode),
            world_names: Some(join_game.dimension_names.clone()),
            dimension: None,
            dimension_codec: None,
            world_name: Some(join_game.dimension_name.clone()),
            hashed_seed: Some(join_game.hashed_seed),
            max_players: Some(join_game.max_players),
            view_distance: Some(join_game.view_distance),
            reduced_debug_info: Some(join_game.reduced_debug_info),
            enable_respawn_screen: Some(join_game.enable_respawn_screen),
            is_debug: Some(join_game.is_debug),
            is_flat: Some(join_game.is_flat),
            simulation_distance: Some(join_game.simulation_distance),
            do_limited_crafting: Some(join_game.do_limited_crafting),
            dimension_type: Some(join_game.dimension_type),
            dimension_name: Some(join_game.dimension_name),
            portal_cooldown: Some(join_game.portal_cooldown),
        }
    }
}

impl From<v1_21_4::game::JoinGame> for JoinGameData {
    fn from(join_game: v1_21_4::game::JoinGame) -> Self {
        Self {
            hardcore: Some(join_game.is_hardcore),
            game_mode: Some(join_game.game_mode),
            previous_game_mode: Some(join_game.previous_game_mode),
            world_names: Some(join_game.dimension_names.clone()),
            dimension: None,
            dimension_codec: None,
            world_name: Some(join_game.dimension_name.clone()),
            hashed_seed: Some(join_game.hashed_seed),
            max_players: Some(join_game.max_players),
            view_distance: Some(join_game.view_distance),
            reduced_debug_info: Some(join_game.reduced_debug_info),
            enable_respawn_screen: Some(join_game.enable_respawn_screen),
            is_debug: Some(join_game.is_debug),
            is_flat: Some(join_game.is_flat),
            simulation_distance: Some(join_game.simulation_distance),
            do_limited_crafting: Some(join_game.do_limited_crafting),
            dimension_type: Some(join_game.dimension_type),
            dimension_name: Some(join_game.dimension_name),
            portal_cooldown: Some(join_game.portal_cooldown),
        }
    }
}

/// Check whether the packet ID matches.
pub fn is_packet(client_info: &ClientInfo, packet_id: u8) -> bool {
    match client_info.protocol() {
        Some(p) if p < v1_17::PROTOCOL => packet_id == v1_16_3::game::JoinGame::PACKET_ID,
        Some(p) if p < 764 => packet_id == v1_17::game::JoinGame::PACKET_ID, // 1.20.2
        Some(p) if p < v1_21::PROTOCOL => packet_id == v1_20_5::game::JoinGame::PACKET_ID,
        Some(p) if p < v1_21_2::PROTOCOL => packet_id == v1_21::game::JoinGame::PACKET_ID,
        Some(p) if p < v1_21_4::PROTOCOL => packet_id == v1_21_2::game::JoinGame::PACKET_ID,
        _ => packet_id == v1_21_4::game::JoinGame::PACKET_ID,
    }
}

/// Send initial join game packet to client for lobby.
#[cfg(feature = "lobby")]
pub async fn lobby_send(
    client: &Client,
    client_info: &ClientInfo,
    writer: &mut WriteHalf<'_>,
    server: &Server,
) -> Result<(), ()> {
    let status = server.status().await;
    let join_game = server.probed_join_game.read().await;

    // Get dimension codec and build lobby dimension
    let dimension_codec: CompoundTag = if let Some(join_game) = join_game.as_ref() {
        join_game
            .dimension_codec
            .clone()
            .unwrap_or_else(dimension::default_dimension_codec)
    } else {
        dimension::default_dimension_codec()
    };

    // Get other values from status and probed join game data
    let dimension: CompoundTag = dimension::lobby_dimension(&dimension_codec);
    let hardcore = join_game.as_ref().and_then(|p| p.hardcore).unwrap_or(false);
    let world_names = join_game
        .as_ref()
        .and_then(|p| p.world_names.clone())
        .unwrap_or_else(|| {
            vec![
                "minecraft:overworld".into(),
                "minecraft:the_nether".into(),
                "minecraft:the_end".into(),
            ]
        });
    let max_players = status
        .as_ref()
        .map(|s| s.players.max as i32)
        .or_else(|| join_game.as_ref().and_then(|p| p.max_players))
        .unwrap_or(20);
    let view_distance = join_game
        .as_ref()
        .and_then(|p| p.view_distance)
        .unwrap_or(10);
    let reduced_debug_info = join_game
        .as_ref()
        .and_then(|p| p.reduced_debug_info)
        .unwrap_or(false);
    let enable_respawn_screen = join_game
        .as_ref()
        .and_then(|p| p.enable_respawn_screen)
        .unwrap_or(true);
    let is_debug = join_game.as_ref().and_then(|p| p.is_debug).unwrap_or(false);
    let is_flat = join_game.as_ref().and_then(|p| p.is_flat).unwrap_or(false);

    match client_info.protocol() {
        Some(p) if p < v1_17::PROTOCOL => {
            packet::write_packet(
                v1_16_3::game::JoinGame {
                    entity_id: 0,
                    hardcore,
                    game_mode: 3,
                    previous_game_mode: -1i8 as u8,
                    world_names,
                    dimension_codec,
                    dimension,
                    world_name: "lazymc:lobby".into(),
                    hashed_seed: 0,
                    max_players,
                    view_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                },
                client,
                writer,
            )
            .await
        }
        Some(p) if p < 764 => { // 1.20.2
            packet::write_packet(
                v1_17::game::JoinGame {
                    entity_id: 0,
                    hardcore,
                    game_mode: 3,
                    previous_game_mode: -1i8 as u8,
                    world_names,
                    dimension_codec,
                    dimension,
                    world_name: "lazymc:lobby".into(),
                    hashed_seed: 0,
                    max_players,
                    view_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                },
                client,
                writer,
            )
            .await
        }
        Some(p) if p < v1_21::PROTOCOL => {
            packet::write_packet(
                v1_20_5::game::JoinGame {
                    entity_id: 0,
                    is_hardcore: hardcore,
                    game_mode: 3,
                    previous_game_mode: 255, // -1 in u8
                    dimension_names: world_names,
                    dimension_type: 0,
                    dimension_name: "lazymc:lobby".into(),
                    hashed_seed: 0,
                    max_players,
                    view_distance,
                    simulation_distance: view_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                    do_limited_crafting: false,
                    death_location: v1_20_5::game::OptionalGlobalPos(None),
                    portal_cooldown: 0,
                },
                client,
                writer,
            )
            .await
        }
        Some(p) if p < v1_21_2::PROTOCOL => {
            packet::write_packet(
                v1_21::game::JoinGame {
                    entity_id: 0,
                    is_hardcore: hardcore,
                    game_mode: 3,
                    previous_game_mode: 255,
                    dimension_names: world_names,
                    dimension_type: 0,
                    dimension_name: "lazymc:lobby".into(),
                    hashed_seed: 0,
                    max_players,
                    view_distance,
                    simulation_distance: view_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                    do_limited_crafting: false,
                    death_location: v1_21::game::OptionalGlobalPos(None),
                    portal_cooldown: 0,
                },
                client,
                writer,
            )
            .await
        }
        Some(p) if p < v1_21_4::PROTOCOL => {
            packet::write_packet(
                v1_21_2::game::JoinGame {
                    entity_id: 0,
                    is_hardcore: hardcore,
                    game_mode: 3,
                    previous_game_mode: 255,
                    dimension_names: world_names,
                    dimension_type: 0,
                    dimension_name: "lazymc:lobby".into(),
                    hashed_seed: 0,
                    max_players,
                    view_distance,
                    simulation_distance: view_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                    do_limited_crafting: false,
                    death_location: v1_21_2::game::OptionalGlobalPos(None),
                    portal_cooldown: 0,
                },
                client,
                writer,
            )
            .await
        }
        _ => {
            packet::write_packet(
                v1_21_4::game::JoinGame {
                    entity_id: 0,
                    is_hardcore: hardcore,
                    game_mode: 3,
                    previous_game_mode: 255,
                    dimension_names: world_names,
                    dimension_type: 0,
                    dimension_name: "lazymc:lobby".into(),
                    hashed_seed: 0,
                    max_players,
                    view_distance,
                    simulation_distance: view_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                    do_limited_crafting: false,
                    death_location: v1_21_4::game::OptionalGlobalPos(None),
                    portal_cooldown: 0,
                },
                client,
                writer,
            )
            .await
        }
    }
}
