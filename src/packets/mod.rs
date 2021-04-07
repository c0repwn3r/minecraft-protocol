pub mod play_clientbound;
pub mod serializer;
pub use minecraft_packet_derive::*;
use serializer::*;

#[derive(Debug)]
pub struct VarInt(pub i32);
#[derive(Debug)]
pub struct VarLong(pub i64);

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Direction {
    South = 1,
    West,
    North,
    East,
}

pub type UUID = u128;
pub type Angle = u8;

/// This is used to replace an unsupported structure by taking all the remaining bytes of a packet.
/// Feel free to make PRs.
#[derive(Debug)]
pub struct RawBytes<'a> {
    data: &'a [u8],
}

#[derive(Debug, MinecraftPacket)]
pub struct TestPacket {
    data: u8,
}
