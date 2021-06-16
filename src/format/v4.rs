use std::io::Read;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_with::serde_as;

use crate::Error;

pub trait Parse: Sized + DeserializeOwned {
    fn parse<R: Read + ?Sized>(reader: &mut R) -> Result<Self, Error>;
}

macro_rules! impl_parse {
    ($ident:ty, $size:expr) => {
        impl Parse for $ident {
            fn parse<R: Read + ?Sized>(reader: &mut R) -> Result<Self, Error> {
                Ok(bincode::deserialize_from(&mut reader.take($size))?)
            }
        }
    };

    ($ident:ty) => {
        impl_parse!($ident, ::std::mem::size_of::<$ident>() as u64);
    };
}

#[repr(C, packed)]
#[serde_as]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DemHeader {
    pub magic: [u8; 8],
    pub demo_protocol: i32,
    pub network_protocol: i32,
    #[serde_as(as = "[_; 260]")]
    pub server_name: [u8; 260],
    #[serde_as(as = "[_; 260]")]
    pub client_name: [u8; 260],
    #[serde_as(as = "[_; 260]")]
    pub map_name: [u8; 260],
    #[serde_as(as = "[_; 260]")]
    pub game_directory: [u8; 260],
    pub playback_time: f32,
    pub playback_ticks: i32,
    pub playback_frames: i32,
    pub signon_length: i32
}
impl_parse!(DemHeader);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SequenceInfo {
    pub sequence_in: i32,
    pub sequence_out: i32
}
impl_parse!(SequenceInfo);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CommandHeader {
    pub command: u8,
    pub tick: i32,
    pub player_slot: u8
}
impl_parse!(CommandHeader);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ViewInfo {
    pub origin: [f32; 3],
    pub angles: [f32; 3],
    pub local_angles: [f32; 3]
}
impl_parse!(ViewInfo);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerViewInfo {
    pub flags: i32,
    pub original: ViewInfo,
    pub resampled: ViewInfo
}
impl_parse!(PlayerViewInfo);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CommandInfo {
    pub players_view_info: [PlayerViewInfo; 2]
}
impl_parse!(CommandInfo);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DataHeader {
    pub size: i32
}
impl_parse!(DataHeader);

#[derive(Clone, Debug, Serialize)]
pub struct ServerClass {
    pub id: u16,
    pub name: String,
    pub datatable: String
}

#[repr(C, packed)]
#[serde_as]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub version: u64,
    pub xuid: u64,
    #[serde_as(as = "[_; 128]")]
    pub name: [u8; 128],
    pub user_id: i32,
    #[serde_as(as = "[_; 33]")]
    pub guid: [u8; 33],
    pub friends_id: u32,
    #[serde_as(as = "[_; 128]")]
    pub friends_name: [u8; 128],
    pub fake_player: bool,
    pub is_hltv: bool,
    pub custom_files: [u64; 4],
    pub files_downloaded: u8,
    pub entity_id: i32
}
impl_parse!(PlayerInfo);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PacketInfo {
    pub command_info: CommandInfo,
    pub sequence_info: SequenceInfo
}
impl_parse!(PacketInfo);
