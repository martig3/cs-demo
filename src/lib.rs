
mod format;
mod util;
mod protos;

use std::io::Read;

use protobuf::{ Message, ProtobufEnum };
use util::{ReadExt, read_varuint};

use format::*;
use protos::netmessages::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub mod events {
    pub use super::format::*;
    pub use super::protos::netmessages::*;
    pub use super::protos::cstrike15_usermessages::*;
}

macro_rules! read_varuint_and_dec {
    ($reader:ident, $size:ident) => {
        {
            let data = read_varuint($reader)?;
            $size -= data.1;
            data.0
        }
    };
}

macro_rules! parse_and_dispatch {
    ($ident:ident, $reader:ident, $dispatcher:ident) => {
        {
            let event = $ident::parse_from_reader($reader)?;
            $dispatcher.dispatch(&event);
        }
    };
}

macro_rules! on_fn {
    ($($ident:ident => $ty:ty);+) => ($(
        fn $ident(&self, _: &$ty) {}
    )+);
}

macro_rules! impl_dispatch {
    ($($ident:ident => $ty:ty);+) => ($(
        impl<E: EventHandler> Dispatcher<$ty> for E {
            fn dispatch(&self, event: &$ty) {
                self.$ident(event);
            }
        }
    )+);
}

pub trait EventHandler {
    on_fn! {
        on_dem_header => DemHeader;
        on_packet_info => PacketInfo;
        on_server_class => ServerClass;

        on_nop => CNETMsg_NOP;
        on_disconnect => CNETMsg_Disconnect;
        on_file => CNETMsg_File;
        on_split_screen_user => CNETMsg_SplitScreenUser;
        on_tick => CNETMsg_Tick;
        on_string_cmd => CNETMsg_StringCmd;
        on_set_con_var => CNETMsg_SetConVar;
        on_signon_state => CNETMsg_SignonState;
        on_player_avatar_data => CNETMsg_PlayerAvatarData;

        on_server_info => CSVCMsg_ServerInfo;
        on_send_table => CSVCMsg_SendTable;
        on_class_info => CSVCMsg_ClassInfo;
        on_set_pause => CSVCMsg_SetPause;
        on_create_string_table => CSVCMsg_CreateStringTable;
        on_update_string_table => CSVCMsg_UpdateStringTable;
        on_voice_init => CSVCMsg_VoiceInit;
        on_voice_data => CSVCMsg_VoiceData;
        on_print => CSVCMsg_Print;
        on_sounds => CSVCMsg_Sounds;
        on_set_view => CSVCMsg_SetView;
        on_fix_angle => CSVCMsg_FixAngle;
        on_crosshair_angle => CSVCMsg_CrosshairAngle;
        on_bspdecal => CSVCMsg_BSPDecal;
        on_split_screen => CSVCMsg_SplitScreen;
        on_user_message => CSVCMsg_UserMessage;
        on_entity_message => CSVCMsg_EntityMsg;
        on_game_event => CSVCMsg_GameEvent;
        on_packet_entities => CSVCMsg_PacketEntities;
        on_temp_entities => CSVCMsg_TempEntities;
        on_prefetch => CSVCMsg_Prefetch;
        on_menu => CSVCMsg_Menu;
        on_game_event_list => CSVCMsg_GameEventList;
        on_get_cvar_value => CSVCMsg_GetCvarValue;
        on_paintmap_data => CSVCMsg_PaintmapData;
        on_cmd_key_values => CSVCMsg_CmdKeyValues;
        on_encrypted_data => CSVCMsg_EncryptedData;
        on_hltv_replay => CSVCMsg_HltvReplay;
        on_broadcast_command => CSVCMsg_Broadcast_Command
    }
}

pub trait Dispatcher<E> {
    fn dispatch(&self, event: &E);
}

impl_dispatch! {
    on_dem_header => DemHeader;
    on_packet_info => PacketInfo;
    on_server_class => ServerClass;

    on_nop => CNETMsg_NOP;
    on_disconnect => CNETMsg_Disconnect;
    on_file => CNETMsg_File;
    on_split_screen_user => CNETMsg_SplitScreenUser;
    on_tick => CNETMsg_Tick;
    on_string_cmd => CNETMsg_StringCmd;
    on_set_con_var => CNETMsg_SetConVar;
    on_signon_state => CNETMsg_SignonState;
    on_player_avatar_data => CNETMsg_PlayerAvatarData;

    on_server_info => CSVCMsg_ServerInfo;
    on_send_table => CSVCMsg_SendTable;
    on_class_info => CSVCMsg_ClassInfo;
    on_set_pause => CSVCMsg_SetPause;
    on_create_string_table => CSVCMsg_CreateStringTable;
    on_update_string_table => CSVCMsg_UpdateStringTable;
    on_voice_init => CSVCMsg_VoiceInit;
    on_voice_data => CSVCMsg_VoiceData;
    on_print => CSVCMsg_Print;
    on_sounds => CSVCMsg_Sounds;
    on_set_view => CSVCMsg_SetView;
    on_fix_angle => CSVCMsg_FixAngle;
    on_crosshair_angle => CSVCMsg_CrosshairAngle;
    on_bspdecal => CSVCMsg_BSPDecal;
    on_split_screen => CSVCMsg_SplitScreen;
    on_user_message => CSVCMsg_UserMessage;
    on_entity_message => CSVCMsg_EntityMsg;
    on_game_event => CSVCMsg_GameEvent;
    on_packet_entities => CSVCMsg_PacketEntities;
    on_temp_entities => CSVCMsg_TempEntities;
    on_prefetch => CSVCMsg_Prefetch;
    on_menu => CSVCMsg_Menu;
    on_game_event_list => CSVCMsg_GameEventList;
    on_get_cvar_value => CSVCMsg_GetCvarValue;
    on_paintmap_data => CSVCMsg_PaintmapData;
    on_cmd_key_values => CSVCMsg_CmdKeyValues;
    on_encrypted_data => CSVCMsg_EncryptedData;
    on_hltv_replay => CSVCMsg_HltvReplay;
    on_broadcast_command => CSVCMsg_Broadcast_Command
}

fn parse_net_command<R: Read + Sized, D: EventHandler>(reader: &mut R, dispatcher: &D, command: NET_Messages) -> Result<(), Error> {
    use NET_Messages::*;

    match command {
        net_NOP => parse_and_dispatch!(CNETMsg_NOP, reader, dispatcher),
        net_Disconnect => parse_and_dispatch!(CNETMsg_Disconnect, reader, dispatcher),
        net_File => parse_and_dispatch!(CNETMsg_File, reader, dispatcher),
        net_SplitScreenUser => parse_and_dispatch!(CNETMsg_SplitScreenUser, reader, dispatcher),
        net_Tick => parse_and_dispatch!(CNETMsg_Tick, reader, dispatcher),
        net_StringCmd => parse_and_dispatch!(CNETMsg_StringCmd, reader, dispatcher),
        net_SetConVar => parse_and_dispatch!(CNETMsg_SetConVar, reader, dispatcher),
        net_SignonState => parse_and_dispatch!(CNETMsg_SignonState, reader, dispatcher),
        net_PlayerAvatarData => parse_and_dispatch!(CNETMsg_PlayerAvatarData, reader, dispatcher),
    }

    assert_eq!(reader.read(&mut [0u8; 1])?, 0);
    Ok(())
}

fn parse_svc_command<R: Read + Sized, D: EventHandler>(reader: &mut R, dispatcher: &D, command: SVC_Messages) -> Result<(), Error> {
    use SVC_Messages::*;

    match command {
        svc_ServerInfo => parse_and_dispatch!(CSVCMsg_ServerInfo, reader, dispatcher),
        svc_SendTable => parse_and_dispatch!(CSVCMsg_SendTable, reader, dispatcher),
        svc_ClassInfo => parse_and_dispatch!(CSVCMsg_ClassInfo, reader, dispatcher),
        svc_SetPause => parse_and_dispatch!(CSVCMsg_SetPause, reader, dispatcher),
        svc_CreateStringTable => parse_and_dispatch!(CSVCMsg_CreateStringTable, reader, dispatcher),
        svc_UpdateStringTable => parse_and_dispatch!(CSVCMsg_UpdateStringTable, reader, dispatcher),
        svc_VoiceInit => parse_and_dispatch!(CSVCMsg_VoiceInit, reader, dispatcher),
        svc_VoiceData => parse_and_dispatch!(CSVCMsg_VoiceData, reader, dispatcher),
        svc_Print => parse_and_dispatch!(CSVCMsg_Print, reader, dispatcher),
        svc_Sounds => parse_and_dispatch!(CSVCMsg_Sounds, reader, dispatcher),
        svc_SetView => parse_and_dispatch!(CSVCMsg_SetView, reader, dispatcher),
        svc_FixAngle => parse_and_dispatch!(CSVCMsg_FixAngle, reader, dispatcher),
        svc_CrosshairAngle => parse_and_dispatch!(CSVCMsg_CrosshairAngle, reader, dispatcher),
        svc_BSPDecal => parse_and_dispatch!(CSVCMsg_BSPDecal, reader, dispatcher),
        svc_SplitScreen => parse_and_dispatch!(CSVCMsg_SplitScreen, reader, dispatcher),
        svc_UserMessage => parse_and_dispatch!(CSVCMsg_UserMessage, reader, dispatcher),
        svc_EntityMessage => parse_and_dispatch!(CSVCMsg_EntityMsg, reader, dispatcher),
        svc_GameEvent => parse_and_dispatch!(CSVCMsg_GameEvent, reader, dispatcher),
        svc_PacketEntities => parse_and_dispatch!(CSVCMsg_PacketEntities, reader, dispatcher),
        svc_TempEntities => parse_and_dispatch!(CSVCMsg_TempEntities, reader, dispatcher),
        svc_Prefetch => parse_and_dispatch!(CSVCMsg_Prefetch, reader, dispatcher),
        svc_Menu => parse_and_dispatch!(CSVCMsg_Menu, reader, dispatcher),
        svc_GameEventList => parse_and_dispatch!(CSVCMsg_GameEventList, reader, dispatcher),
        svc_GetCvarValue => parse_and_dispatch!(CSVCMsg_GetCvarValue, reader, dispatcher),
        svc_PaintmapData => parse_and_dispatch!(CSVCMsg_PaintmapData, reader, dispatcher),
        svc_CmdKeyValues => parse_and_dispatch!(CSVCMsg_CmdKeyValues, reader, dispatcher),
        svc_EncryptedData => parse_and_dispatch!(CSVCMsg_EncryptedData, reader, dispatcher),
        svc_HltvReplay => parse_and_dispatch!(CSVCMsg_HltvReplay, reader, dispatcher),
        svc_Broadcast_Command => parse_and_dispatch!(CSVCMsg_Broadcast_Command, reader, dispatcher),
    }

    assert_eq!(reader.read(&mut [0u8; 1])?, 0);
    Ok(())
}

fn parse_command<R: Read + Sized, D: EventHandler>(reader: &mut R, dispatcher: &D, command: i32) -> Result<(), Error> {
    if let Some(command) = NET_Messages::from_i32(command) {
        parse_net_command(reader, dispatcher, command)
    } else if let Some(command) = SVC_Messages::from_i32(command) {
        parse_svc_command(reader, dispatcher, command)
    } else {
        Err(format!("Invalid Command {}", command).into())
    }
}

fn parse_packet<R: Read + Sized, D: EventHandler>(reader: &mut R, dispatcher: &D) -> Result<(), Error> {
    let packet_info = PacketInfo::parse(reader)?;
    dispatcher.dispatch(&packet_info);
    let data_header = DataHeader::parse(reader)?;

    let mut packet_size = data_header.size as usize;
    let reader = &mut reader.take(data_header.size as u64);

    while packet_size > 0 {
        let command = read_varuint_and_dec!(reader, packet_size) as i32;
        let size = read_varuint_and_dec!(reader, packet_size) as usize;
        let reader = &mut reader.take(size as u64);

        parse_command(reader, dispatcher, command)?;
        packet_size -= size;
    }

    assert_eq!(reader.read(&mut [0u8; 1])?, 0);
    Ok(())
}

fn parse_datatables<R: Read + Sized, D: EventHandler>(reader: &mut R, dispatcher: &D) -> Result<(), Error> {
    let data_header = DataHeader::parse(reader)?;
    let mut data_size = data_header.size as usize;

    let reader = &mut reader.take(data_size as u64);

    while data_size > 0 {
        let _datatable_type = read_varuint_and_dec!(reader, data_size);
        let datatable_size = read_varuint_and_dec!(reader, data_size) as usize;

        let message = CSVCMsg_SendTable::parse_from_reader(&mut reader.take(datatable_size as u64))?;
        dispatcher.dispatch(&message);

        data_size -= datatable_size;

        if message.get_is_end() {
            break;
        }
    }

    let reader = &mut reader.take(data_size as u64);
    
    // Valve, why?
    let server_classes = reader.read_u16_le()?;
    for _ in 0..server_classes {
        let id = reader.read_u16_le()?;

        let name =  {
            let mut string_buffer = Vec::with_capacity(256);
            let mut byte = reader.read_u8()?;
            while byte != 0 {
                string_buffer.push(byte);
                byte = reader.read_u8()?;
            }
            String::from_utf8(string_buffer)?
        };
        let datatable = {
            let mut string_buffer = Vec::with_capacity(256);
            let mut byte = reader.read_u8()?;
            while byte != 0 {
                string_buffer.push(byte);
                byte = reader.read_u8()?;
            }
            String::from_utf8(string_buffer)?
        };

        let server_class = ServerClass {
            id,
            name,
            datatable
        };

        dispatcher.dispatch(&server_class);
    }

    assert_eq!(reader.read(&mut [0u8; 1])?, 0);
    Ok(())
}

pub fn parse_dem_file<R: Read + Sized, D: EventHandler>(reader: &mut R, dispatcher: &D) -> Result<(), Error> {
    let header = DemHeader::parse(reader)?;
    dispatcher.dispatch(&header);

    loop {
        let command_header = CommandHeader::parse(reader)?;

        match command_header.command {
            // dem_signon | dem_packet
            1 | 2 => parse_packet(reader, dispatcher)?,

            // dem_synctick
            3 => {},

            // dem_consolecmd
            4 => unimplemented!(),

            // dem_usercmd
            5 => unimplemented!(),

            // dem_datatables
            6 => parse_datatables(reader, dispatcher)?,

            // dem_stop
            7 => {
                assert_eq!(reader.read(&mut [0u8; 1])?, 0);
                break;
            },

            // dem_customdata
            8 => unimplemented!(),

            // dem_stringtables
            9 => unimplemented!(),

            command => {
                eprintln!("Unknown command: {}", command);
                break;
            }
        }
    }

    Ok(())
}
