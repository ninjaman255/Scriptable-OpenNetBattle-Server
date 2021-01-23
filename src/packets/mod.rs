mod client_packets;
pub use client_packets::*;

mod server_packets;
pub use server_packets::*;

pub(crate) const TILE_WIDTH: f64 = 62.0 + 2.5;
pub(crate) const TILE_HEIGHT: f64 = 32.0 + 0.5;
