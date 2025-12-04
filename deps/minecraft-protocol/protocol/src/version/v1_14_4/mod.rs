// Spec: https://wiki.vg/index.php?title=Protocol&oldid=15346

pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

/// Minecraft protocol version.
pub const PROTOCOL: u32 = 498;
