
use protobuf::Message;
use protobuf::ProtobufEnum;

pub use super::format::*;
pub use super::protos::netmessages::*;
pub use super::protos::cstrike15_usermessages::*;

use crate::Error;

macro_rules! impl_dispatch {
    ($handler:path; $($ident:ident => $ty:ty);+) => ($(
        impl<E: $handler> Dispatcher<$ty> for E {
            #[inline(always)]
            fn dispatch(&self, event: &$ty) -> Result<(), Error> {
                self.$ident(event)
            }
        }
    )+);
}

macro_rules! on_fn {
    ($($ident:ident => $ty:ty);+) => ($(
        #[inline]
        fn $ident(&self, _: &$ty) -> Result<(), Error> { Ok(()) }
    )+);
}

macro_rules! forward_to_inner {
    ($($ident:ident => $ty:ty);+) => ($(
        fn $ident(&self, event: &$ty) -> Result<(), Error> {
            self.0.$ident(event)
        }
    )+);
}

macro_rules! parse_and_dispatch {
    ($ident:ident, $data:ident, $dispatcher:expr) => {
        {
            let event = $ident::parse_from_bytes($data)?;
            $dispatcher.dispatch(&event)?;
        }
    };
}

pub trait Dispatcher<E> {
    fn dispatch(&self, event: &E) -> Result<(), Error>;
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

pub trait UserMessageEventHandler: EventHandler {
    on_fn! {
        on_vguimenu => CCSUsrMsg_VGUIMenu;
        on_geiger => CCSUsrMsg_Geiger;
        on_train => CCSUsrMsg_Train;
        on_hud_text => CCSUsrMsg_HudText;
        on_say_text => CCSUsrMsg_SayText;
        on_say_text2 => CCSUsrMsg_SayText2;
        on_text_msg => CCSUsrMsg_TextMsg;
        on_hud_msg => CCSUsrMsg_HudMsg;
        on_reset_hud => CCSUsrMsg_ResetHud;
        on_game_title => CCSUsrMsg_GameTitle;
        on_shake => CCSUsrMsg_Shake;
        on_fade => CCSUsrMsg_Fade;
        on_rumble => CCSUsrMsg_Rumble;
        on_close_caption => CCSUsrMsg_CloseCaption;
        on_close_caption_direct => CCSUsrMsg_CloseCaptionDirect;
        on_send_audio => CCSUsrMsg_SendAudio;
        on_raw_audio => CCSUsrMsg_RawAudio;
        on_voice_mask => CCSUsrMsg_VoiceMask;
        on_request_state => CCSUsrMsg_RequestState;
        on_damage => CCSUsrMsg_Damage;
        on_radio_text => CCSUsrMsg_RadioText;
        on_hint_text => CCSUsrMsg_HintText;
        on_key_hint_text => CCSUsrMsg_KeyHintText;
        on_process_spotted_entity_update => CCSUsrMsg_ProcessSpottedEntityUpdate;
        on_reload_effect => CCSUsrMsg_ReloadEffect;
        on_adjust_money => CCSUsrMsg_AdjustMoney;
        //on_UpdateTeamMoney => CCSUsrMsg_UpdateTeamMoney;
        on_stop_spectator_mode => CCSUsrMsg_StopSpectatorMode;
        on_kill_cam => CCSUsrMsg_KillCam;
        on_desired_timescale => CCSUsrMsg_DesiredTimescale;
        on_current_timescale => CCSUsrMsg_CurrentTimescale;
        on_achievement_event => CCSUsrMsg_AchievementEvent;
        on_match_end_conditions => CCSUsrMsg_MatchEndConditions;
        on_disconnect_to_lobby => CCSUsrMsg_DisconnectToLobby;
        on_player_stats_update => CCSUsrMsg_PlayerStatsUpdate;
        on_display_inventory => CCSUsrMsg_DisplayInventory;
        on_warmup_has_ended => CCSUsrMsg_WarmupHasEnded;
        on_client_info => CCSUsrMsg_ClientInfo;
        on_xrank_get => CCSUsrMsg_XRankGet;
        on_xrank_upd => CCSUsrMsg_XRankUpd;
        on_call_vote_failed => CCSUsrMsg_CallVoteFailed;
        on_vote_start => CCSUsrMsg_VoteStart;
        on_vote_pass => CCSUsrMsg_VotePass;
        on_vote_failed => CCSUsrMsg_VoteFailed;
        on_vote_setup => CCSUsrMsg_VoteSetup;
        on_server_rank_reveal_all => CCSUsrMsg_ServerRankRevealAll;
        on_send_last_killer_damage_to_client => CCSUsrMsg_SendLastKillerDamageToClient;
        on_server_rank_update => CCSUsrMsg_ServerRankUpdate;
        on_item_pickup => CCSUsrMsg_ItemPickup;
        on_show_menu => CCSUsrMsg_ShowMenu;
        on_bar_time => CCSUsrMsg_BarTime;
        on_ammo_denied => CCSUsrMsg_AmmoDenied;
        on_mark_achievement => CCSUsrMsg_MarkAchievement;
        on_match_stats_update => CCSUsrMsg_MatchStatsUpdate;
        on_item_drop => CCSUsrMsg_ItemDrop;
        on_glow_prop_turn_off => CCSUsrMsg_GlowPropTurnOff;
        on_send_player_item_drops => CCSUsrMsg_SendPlayerItemDrops;
        on_round_backup_filenames => CCSUsrMsg_RoundBackupFilenames;
        on_send_player_item_found => CCSUsrMsg_SendPlayerItemFound;
        on_report_hit => CCSUsrMsg_ReportHit;
        on_xp_update => CCSUsrMsg_XpUpdate;
        on_quest_progress => CCSUsrMsg_QuestProgress;
        on_score_leaderboard_data => CCSUsrMsg_ScoreLeaderboardData;
        on_player_decal_digital_signature => CCSUsrMsg_PlayerDecalDigitalSignature;
        on_weapon_sound => CCSUsrMsg_WeaponSound;
        on_update_screen_health_bar => CCSUsrMsg_UpdateScreenHealthBar;
        on_entity_outline_highlight => CCSUsrMsg_EntityOutlineHighlight;
        on_ssui => CCSUsrMsg_SSUI;
        on_survival_stats => CCSUsrMsg_SurvivalStats;
        //on_DisconnectToLobby2 => CCSUsrMsg_DisconnectToLobby2;
        on_end_of_match_all_players_data => CCSUsrMsg_EndOfMatchAllPlayersData;
        on_round_impact_score_data => CCSUsrMsg_RoundImpactScoreData;
        on_current_round_odds => CCSUsrMsg_CurrentRoundOdds;
        on_deep_stats => CCSUsrMsg_DeepStats
    }
}

pub struct UserMessageDecoder<T>(pub T);

impl<T: UserMessageEventHandler> EventHandler for UserMessageDecoder<T> {
    fn on_user_message(&self, event: &CSVCMsg_UserMessage) -> Result<(), Error> {
        let command = event.get_msg_type();
        let data = event.get_msg_data();

        if let Some(command) = ECstrike15UserMessages::from_i32(command) {
            use ECstrike15UserMessages::*;

            match command {
                CS_UM_VGUIMenu => parse_and_dispatch!(CCSUsrMsg_VGUIMenu, data, self.0),
                CS_UM_Geiger => parse_and_dispatch!(CCSUsrMsg_Geiger, data, self.0),
                CS_UM_Train => parse_and_dispatch!(CCSUsrMsg_Train, data, self.0),
                CS_UM_HudText => parse_and_dispatch!(CCSUsrMsg_HudText, data, self.0),
                CS_UM_SayText => parse_and_dispatch!(CCSUsrMsg_SayText, data, self.0),
                CS_UM_SayText2 => parse_and_dispatch!(CCSUsrMsg_SayText2, data, self.0),
                CS_UM_TextMsg => parse_and_dispatch!(CCSUsrMsg_TextMsg, data, self.0),
                CS_UM_HudMsg => parse_and_dispatch!(CCSUsrMsg_HudMsg, data, self.0),
                CS_UM_ResetHud => parse_and_dispatch!(CCSUsrMsg_ResetHud, data, self.0),
                CS_UM_GameTitle => parse_and_dispatch!(CCSUsrMsg_GameTitle, data, self.0),
                CS_UM_Shake => parse_and_dispatch!(CCSUsrMsg_Shake, data, self.0),
                CS_UM_Fade => parse_and_dispatch!(CCSUsrMsg_Fade, data, self.0),
                CS_UM_Rumble => parse_and_dispatch!(CCSUsrMsg_Rumble, data, self.0),
                CS_UM_CloseCaption => parse_and_dispatch!(CCSUsrMsg_CloseCaption, data, self.0),
                CS_UM_CloseCaptionDirect => parse_and_dispatch!(CCSUsrMsg_CloseCaptionDirect, data, self.0),
                CS_UM_SendAudio => parse_and_dispatch!(CCSUsrMsg_SendAudio, data, self.0),
                CS_UM_RawAudio => parse_and_dispatch!(CCSUsrMsg_RawAudio, data, self.0),
                CS_UM_VoiceMask => parse_and_dispatch!(CCSUsrMsg_VoiceMask, data, self.0),
                CS_UM_RequestState => parse_and_dispatch!(CCSUsrMsg_RequestState, data, self.0),
                CS_UM_Damage => parse_and_dispatch!(CCSUsrMsg_Damage, data, self.0),
                CS_UM_RadioText => parse_and_dispatch!(CCSUsrMsg_RadioText, data, self.0),
                CS_UM_HintText => parse_and_dispatch!(CCSUsrMsg_HintText, data, self.0),
                CS_UM_KeyHintText => parse_and_dispatch!(CCSUsrMsg_KeyHintText, data, self.0),
                CS_UM_ProcessSpottedEntityUpdate => parse_and_dispatch!(CCSUsrMsg_ProcessSpottedEntityUpdate, data, self.0),
                CS_UM_ReloadEffect => parse_and_dispatch!(CCSUsrMsg_ReloadEffect, data, self.0),
                CS_UM_AdjustMoney => parse_and_dispatch!(CCSUsrMsg_AdjustMoney, data, self.0),
                CS_UM_UpdateTeamMoney => todo!(),
                CS_UM_StopSpectatorMode => parse_and_dispatch!(CCSUsrMsg_StopSpectatorMode, data, self.0),
                CS_UM_KillCam => parse_and_dispatch!(CCSUsrMsg_KillCam, data, self.0),
                CS_UM_DesiredTimescale => parse_and_dispatch!(CCSUsrMsg_DesiredTimescale, data, self.0),
                CS_UM_CurrentTimescale => parse_and_dispatch!(CCSUsrMsg_CurrentTimescale, data, self.0),
                CS_UM_AchievementEvent => parse_and_dispatch!(CCSUsrMsg_AchievementEvent, data, self.0),
                CS_UM_MatchEndConditions => parse_and_dispatch!(CCSUsrMsg_MatchEndConditions, data, self.0),
                CS_UM_DisconnectToLobby => parse_and_dispatch!(CCSUsrMsg_DisconnectToLobby, data, self.0),
                CS_UM_PlayerStatsUpdate => parse_and_dispatch!(CCSUsrMsg_PlayerStatsUpdate, data, self.0),
                CS_UM_DisplayInventory => parse_and_dispatch!(CCSUsrMsg_DisplayInventory, data, self.0),
                CS_UM_WarmupHasEnded => parse_and_dispatch!(CCSUsrMsg_WarmupHasEnded, data, self.0),
                CS_UM_ClientInfo => parse_and_dispatch!(CCSUsrMsg_ClientInfo, data, self.0),
                CS_UM_XRankGet => parse_and_dispatch!(CCSUsrMsg_XRankGet, data, self.0),
                CS_UM_XRankUpd => parse_and_dispatch!(CCSUsrMsg_XRankUpd, data, self.0),
                CS_UM_CallVoteFailed => parse_and_dispatch!(CCSUsrMsg_CallVoteFailed, data, self.0),
                CS_UM_VoteStart => parse_and_dispatch!(CCSUsrMsg_VoteStart, data, self.0),
                CS_UM_VotePass => parse_and_dispatch!(CCSUsrMsg_VotePass, data, self.0),
                CS_UM_VoteFailed => parse_and_dispatch!(CCSUsrMsg_VoteFailed, data, self.0),
                CS_UM_VoteSetup => parse_and_dispatch!(CCSUsrMsg_VoteSetup, data, self.0),
                CS_UM_ServerRankRevealAll => parse_and_dispatch!(CCSUsrMsg_ServerRankRevealAll, data, self.0),
                CS_UM_SendLastKillerDamageToClient => parse_and_dispatch!(CCSUsrMsg_SendLastKillerDamageToClient, data, self.0),
                CS_UM_ServerRankUpdate => parse_and_dispatch!(CCSUsrMsg_ServerRankUpdate, data, self.0),
                CS_UM_ItemPickup => parse_and_dispatch!(CCSUsrMsg_ItemPickup, data, self.0),
                CS_UM_ShowMenu => parse_and_dispatch!(CCSUsrMsg_ShowMenu, data, self.0),
                CS_UM_BarTime => parse_and_dispatch!(CCSUsrMsg_BarTime, data, self.0),
                CS_UM_AmmoDenied => parse_and_dispatch!(CCSUsrMsg_AmmoDenied, data, self.0),
                CS_UM_MarkAchievement => parse_and_dispatch!(CCSUsrMsg_MarkAchievement, data, self.0),
                CS_UM_MatchStatsUpdate => parse_and_dispatch!(CCSUsrMsg_MatchStatsUpdate, data, self.0),
                CS_UM_ItemDrop => parse_and_dispatch!(CCSUsrMsg_ItemDrop, data, self.0),
                CS_UM_GlowPropTurnOff => parse_and_dispatch!(CCSUsrMsg_GlowPropTurnOff, data, self.0),
                CS_UM_SendPlayerItemDrops => parse_and_dispatch!(CCSUsrMsg_SendPlayerItemDrops, data, self.0),
                CS_UM_RoundBackupFilenames => parse_and_dispatch!(CCSUsrMsg_RoundBackupFilenames, data, self.0),
                CS_UM_SendPlayerItemFound => parse_and_dispatch!(CCSUsrMsg_SendPlayerItemFound, data, self.0),
                CS_UM_ReportHit => parse_and_dispatch!(CCSUsrMsg_ReportHit, data, self.0),
                CS_UM_XpUpdate => parse_and_dispatch!(CCSUsrMsg_XpUpdate, data, self.0),
                CS_UM_QuestProgress => parse_and_dispatch!(CCSUsrMsg_QuestProgress, data, self.0),
                CS_UM_ScoreLeaderboardData => parse_and_dispatch!(CCSUsrMsg_ScoreLeaderboardData, data, self.0),
                CS_UM_PlayerDecalDigitalSignature => parse_and_dispatch!(CCSUsrMsg_PlayerDecalDigitalSignature, data, self.0),
                CS_UM_WeaponSound => parse_and_dispatch!(CCSUsrMsg_WeaponSound, data, self.0),
                CS_UM_UpdateScreenHealthBar => parse_and_dispatch!(CCSUsrMsg_UpdateScreenHealthBar, data, self.0),
                CS_UM_EntityOutlineHighlight => parse_and_dispatch!(CCSUsrMsg_EntityOutlineHighlight, data, self.0),
                CS_UM_SSUI => parse_and_dispatch!(CCSUsrMsg_SSUI, data, self.0),
                CS_UM_SurvivalStats => parse_and_dispatch!(CCSUsrMsg_SurvivalStats, data, self.0),
                CS_UM_DisconnectToLobby2 => todo!(),
                CS_UM_EndOfMatchAllPlayersData => parse_and_dispatch!(CCSUsrMsg_EndOfMatchAllPlayersData, data, self.0),
                CS_UM_RoundImpactScoreData => parse_and_dispatch!(CCSUsrMsg_RoundImpactScoreData, data, self.0),
                CS_UM_CurrentRoundOdds => parse_and_dispatch!(CCSUsrMsg_CurrentRoundOdds, data, self.0),
                CS_UM_DeepStats => parse_and_dispatch!(CCSUsrMsg_DeepStats, data, self.0)
            }
        } else {
            self.0.on_user_message(event)?;
        }
        Ok(())
    }

    forward_to_inner! {
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

impl_dispatch! {
    EventHandler;

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

impl_dispatch! {
    UserMessageEventHandler;

    on_vguimenu => CCSUsrMsg_VGUIMenu;
    on_geiger => CCSUsrMsg_Geiger;
    on_train => CCSUsrMsg_Train;
    on_hud_text => CCSUsrMsg_HudText;
    on_say_text => CCSUsrMsg_SayText;
    on_say_text2 => CCSUsrMsg_SayText2;
    on_text_msg => CCSUsrMsg_TextMsg;
    on_hud_msg => CCSUsrMsg_HudMsg;
    on_reset_hud => CCSUsrMsg_ResetHud;
    on_game_title => CCSUsrMsg_GameTitle;
    on_shake => CCSUsrMsg_Shake;
    on_fade => CCSUsrMsg_Fade;
    on_rumble => CCSUsrMsg_Rumble;
    on_close_caption => CCSUsrMsg_CloseCaption;
    on_close_caption_direct => CCSUsrMsg_CloseCaptionDirect;
    on_send_audio => CCSUsrMsg_SendAudio;
    on_raw_audio => CCSUsrMsg_RawAudio;
    on_voice_mask => CCSUsrMsg_VoiceMask;
    on_request_state => CCSUsrMsg_RequestState;
    on_damage => CCSUsrMsg_Damage;
    on_radio_text => CCSUsrMsg_RadioText;
    on_hint_text => CCSUsrMsg_HintText;
    on_key_hint_text => CCSUsrMsg_KeyHintText;
    on_process_spotted_entity_update => CCSUsrMsg_ProcessSpottedEntityUpdate;
    on_reload_effect => CCSUsrMsg_ReloadEffect;
    on_adjust_money => CCSUsrMsg_AdjustMoney;
    //on_UpdateTeamMoney => CCSUsrMsg_UpdateTeamMoney;
    on_stop_spectator_mode => CCSUsrMsg_StopSpectatorMode;
    on_kill_cam => CCSUsrMsg_KillCam;
    on_desired_timescale => CCSUsrMsg_DesiredTimescale;
    on_current_timescale => CCSUsrMsg_CurrentTimescale;
    on_achievement_event => CCSUsrMsg_AchievementEvent;
    on_match_end_conditions => CCSUsrMsg_MatchEndConditions;
    on_disconnect_to_lobby => CCSUsrMsg_DisconnectToLobby;
    on_player_stats_update => CCSUsrMsg_PlayerStatsUpdate;
    on_display_inventory => CCSUsrMsg_DisplayInventory;
    on_warmup_has_ended => CCSUsrMsg_WarmupHasEnded;
    on_client_info => CCSUsrMsg_ClientInfo;
    on_xrank_get => CCSUsrMsg_XRankGet;
    on_xrank_upd => CCSUsrMsg_XRankUpd;
    on_call_vote_failed => CCSUsrMsg_CallVoteFailed;
    on_vote_start => CCSUsrMsg_VoteStart;
    on_vote_pass => CCSUsrMsg_VotePass;
    on_vote_failed => CCSUsrMsg_VoteFailed;
    on_vote_setup => CCSUsrMsg_VoteSetup;
    on_server_rank_reveal_all => CCSUsrMsg_ServerRankRevealAll;
    on_send_last_killer_damage_to_client => CCSUsrMsg_SendLastKillerDamageToClient;
    on_server_rank_update => CCSUsrMsg_ServerRankUpdate;
    on_item_pickup => CCSUsrMsg_ItemPickup;
    on_show_menu => CCSUsrMsg_ShowMenu;
    on_bar_time => CCSUsrMsg_BarTime;
    on_ammo_denied => CCSUsrMsg_AmmoDenied;
    on_mark_achievement => CCSUsrMsg_MarkAchievement;
    on_match_stats_update => CCSUsrMsg_MatchStatsUpdate;
    on_item_drop => CCSUsrMsg_ItemDrop;
    on_glow_prop_turn_off => CCSUsrMsg_GlowPropTurnOff;
    on_send_player_item_drops => CCSUsrMsg_SendPlayerItemDrops;
    on_round_backup_filenames => CCSUsrMsg_RoundBackupFilenames;
    on_send_player_item_found => CCSUsrMsg_SendPlayerItemFound;
    on_report_hit => CCSUsrMsg_ReportHit;
    on_xp_update => CCSUsrMsg_XpUpdate;
    on_quest_progress => CCSUsrMsg_QuestProgress;
    on_score_leaderboard_data => CCSUsrMsg_ScoreLeaderboardData;
    on_player_decal_digital_signature => CCSUsrMsg_PlayerDecalDigitalSignature;
    on_weapon_sound => CCSUsrMsg_WeaponSound;
    on_update_screen_health_bar => CCSUsrMsg_UpdateScreenHealthBar;
    on_entity_outline_highlight => CCSUsrMsg_EntityOutlineHighlight;
    on_ssui => CCSUsrMsg_SSUI;
    on_survival_stats => CCSUsrMsg_SurvivalStats;
    //on_DisconnectToLobby2 => CCSUsrMsg_DisconnectToLobby2;
    on_end_of_match_all_players_data => CCSUsrMsg_EndOfMatchAllPlayersData;
    on_round_impact_score_data => CCSUsrMsg_RoundImpactScoreData;
    on_current_round_odds => CCSUsrMsg_CurrentRoundOdds;
    on_deep_stats => CCSUsrMsg_DeepStats
}
