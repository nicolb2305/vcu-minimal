use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsActiveBoosts {
    pub summoner_id: u64,
    pub ip_boost_end_date: String,
    pub ip_boost_per_win_count: u32,
    pub ip_loyalty_boost: u32,
    pub xp_boost_end_date: String,
    pub xp_boost_per_win_count: u32,
    pub xp_loyalty_boost: u32,
    pub first_win_of_the_day_start_time: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendResource {
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub game_name: String,
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    pub status_message: String,
    pub note: String,
    pub last_seen_online_timestamp: Option<String>,
    pub is_p2_p_conversation_muted: bool,
    pub group_id: u32,
    pub display_group_id: u32,
    pub group_name: String,
    pub display_group_name: String,
    pub lol: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSession {
    pub game_id: u64,
    pub timer: LolChampSelectChampSelectTimer,
    pub chat_details: LolChampSelectChampSelectChatRoomDetails,
    pub my_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub their_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub trades: Vec<LolChampSelectChampSelectTradeContract>,
    pub pick_order_swaps: Vec<LolChampSelectChampSelectSwapContract>,
    pub actions: Vec<Vec<ChampSelectActorType>>,
    pub bans: LolChampSelectChampSelectBannedChampions,
    pub local_player_cell_id: i64,
    pub is_spectating: bool,
    pub allow_skin_selection: bool,
    pub allow_duplicate_picks: bool,
    pub allow_battle_boost: bool,
    pub boostable_skin_count: i32,
    pub allow_rerolling: bool,
    pub rerolls_remaining: u32,
    pub allow_locked_events: bool,
    pub locked_event_index: i32,
    pub bench_enabled: bool,
    pub bench_champions: Vec<LolChampSelectBenchChampion>,
    pub entitled_feature_state: LolChampSelectEntitledFeatureState,
    pub counter: i64,
    pub recovery_counter: i64,
    pub skip_champion_select: bool,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
    pub is_custom_game: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTimer {
    pub adjusted_time_left_in_phase: i64,
    pub total_time_in_phase: i64,
    pub phase: String,
    pub is_infinite: bool,
    pub internal_now_in_epoch_ms: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectChatRoomDetails {
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: LolChampSelectMucJwtDto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPlayerSelection {
    pub cell_id: i64,
    pub champion_id: i32,
    pub selected_skin_id: i32,
    pub ward_skin_id: i64,
    pub spell1_id: u64,
    pub spell2_id: u64,
    pub team: i32,
    pub assigned_position: String,
    pub champion_pick_intent: i32,
    pub summoner_id: u64,
    pub puuid: String,
    pub entitled_feature_type: String,
    pub name_visibility_type: String,
    pub obfuscated_summoner_id: u64,
    pub obfuscated_puuid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTradeContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolChampSelectChampSelectTradeState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum LolChampSelectChampSelectTradeState {
    #[default]
    AVAILABLE = 1,
    BUSY = 2,
    INVALID = 3,
    RECEIVED = 4,
    SENT = 5,
    DECLINED = 6,
    CANCELLED = 7,
    ACCEPTED = 8,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSwapContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolChampSelectChampSelectSwapState,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum LolChampSelectChampSelectSwapState {
    #[default]
    AVAILABLE = 1,
    BUSY = 2,
    INVALID = 3,
    RECEIVED = 4,
    SENT = 5,
    DECLINED = 6,
    CANCELLED = 7,
    ACCEPTED = 8,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectBannedChampions {
    pub my_team_bans: Vec<i32>,
    pub their_team_bans: Vec<i32>,
    pub num_bans: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectBenchChampion {
    pub champion_id: i32,
    pub is_priority: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectEntitledFeatureState {
    pub additional_rerolls: u32,
    pub unlocked_skin_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectAction {
    pub id: i64,
    pub actor_cell_id: i64,
    pub champion_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub completed: bool,
    pub is_ally_action: bool,
    pub is_in_progress: bool,
    pub pick_turn: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectAction {
    pub id: i64,
    pub actor_cell_id: i64,
    pub champion_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub completed: bool,
    pub is_ally_action: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectAction {
    pub id: i64,
    pub actor_cell_id: i64,
    pub champion_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub completed: bool,
    pub is_ally_action: bool,
    pub is_in_progress: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampGridChampion {
    pub id: i32,
    pub name: String,
    pub square_portrait_path: String,
    pub free_to_play: bool,
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub free_to_play_for_queue: bool,
    pub owned: bool,
    pub rented: bool,
    pub disabled: bool,
    pub roles: Vec<String>,
    pub mastery_points: i32,
    pub mastery_level: i32,
    pub mastery_chest_granted: bool,
    pub selection_status: LolChampSelectChampionSelection,
    pub positions_favorited: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampionSelection {
    pub selected_by_me: bool,
    pub ban_intented_by_me: bool,
    pub ban_intented: bool,
    pub is_banned: bool,
    pub pick_intented: bool,
    pub pick_intented_by_me: bool,
    pub pick_intented_position: String,
    pub picked_by_other_or_banned: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChampSelectActorType {
    Option1(LolChampSelectChampSelectAction),
    Option2(LolChampSelectLegacyChampSelectAction),
    Option3(LolLobbyTeamBuilderChampSelectAction),
}

impl ChampSelectActorType {
    pub fn cell_id(&self) -> i64 {
        match self {
            Self::Option1(v) => v.actor_cell_id,
            Self::Option2(v) => v.actor_cell_id,
            Self::Option3(v) => v.actor_cell_id,
        }
    }

    pub fn id(&self) -> i64 {
        match self {
            Self::Option1(v) => v.id,
            Self::Option2(v) => v.id,
            Self::Option3(v) => v.id,
        }
    }

    pub fn champion_id(&self) -> i32 {
        match self {
            Self::Option1(v) => v.champion_id,
            Self::Option2(v) => v.champion_id,
            Self::Option3(v) => v.champion_id,
        }
    }
}
