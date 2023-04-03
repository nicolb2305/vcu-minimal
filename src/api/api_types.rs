use std::{collections::HashMap, vec};

use serde::{Deserialize, Serialize};

use super::data::{Champion, SummonerSpell};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsActiveBoosts {
    pub summoner_id: SummonerId,
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
pub struct SummonerId(u64);

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendResource {
    pub summoner_id: SummonerId,
    pub id: String,
    pub name: SummonerName,
    pub pid: Pid,
    pub puuid: Puuid,
    pub game_name: GameName,
    pub game_tag: GameTag,
    pub icon: SummonerIcon,
    pub availability: ChatAvailability,
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
pub enum ChatAvailability {
    #[default]
    Offline,
    Chat,
    Away,
    Mobile,
    Dnd,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SummonerIcon(i32);

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SummonerName(String);

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameName(String);

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameTag(String);

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pid(String);

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puuid(String);

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSession {
    pub game_id: GameId,
    pub timer: LolChampSelectChampSelectTimer,
    pub chat_details: LolChampSelectChampSelectChatRoomDetails,
    pub my_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub their_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub trades: Vec<LolChampSelectChampSelectTradeContract>,
    pub pick_order_swaps: Vec<LolChampSelectChampSelectSwapContract>,
    pub actions: Vec<Vec<LolChampSelectChampSelectAction>>,
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
pub struct GameId(u64);

impl LolChampSelectChampSelectSession {
    pub fn champions(&self) -> Vec<Vec<Champion>> {
        let my_team: Vec<_> = self.my_team.iter().map(|x| x.champion).collect();
        let their_team: Vec<_> = self.their_team.iter().map(|x| x.champion).collect();
        vec![my_team, their_team]
    }
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChampSelectPhase {
    #[default]
    BanPick,
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
    #[serde(rename = "championId")]
    pub champion: Champion,
    pub selected_skin_id: SkinId,
    pub ward_skin_id: WardSkinId,
    pub spell1_id: SummonerSpell,
    pub spell2_id: SummonerSpell,
    pub team: i32,
    pub assigned_position: String,
    pub champion_pick_intent: i32,
    pub summoner_id: SummonerId,
    pub puuid: Puuid,
    pub entitled_feature_type: String,
    pub name_visibility_type: String,
    pub obfuscated_summoner_id: SummonerId,
    pub obfuscated_puuid: Puuid,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SkinId(i32);

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WardSkinId(i32);

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
    pub my_team_bans: Vec<Champion>,
    pub their_team_bans: Vec<Champion>,
    pub num_bans: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectBenchChampion {
    #[serde(rename = "championId")]
    pub champion: Champion,
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
pub struct LolChampSelectChampSelectAction {
    pub id: Option<i64>,
    pub actor_cell_id: Option<i64>,
    #[serde(rename = "championId")]
    pub champion: Option<Champion>,
    #[serde(rename = "type")]
    pub action_type: Option<String>,
    pub completed: Option<bool>,
    pub is_ally_action: Option<bool>,
    pub is_in_progress: Option<bool>,
    pub pick_turn: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampGridChampion {
    #[serde(rename = "id")]
    pub champion: Champion,
    pub name: String,
    pub square_portrait_path: Path,
    pub free_to_play: bool,
    pub loyalty_reward: bool,
    pub xbox_g_p_reward: bool,
    pub free_to_play_for_queue: bool,
    pub owned: bool,
    pub rented: bool,
    pub disabled: bool,
    pub roles: Vec<LolChampSelectRoles>,
    pub mastery_points: i32,
    pub mastery_level: i32,
    pub mastery_chest_granted: bool,
    pub selection_status: LolChampSelectChampionSelection,
    pub positions_favorited: Vec<LolChampSelectPosition>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum LolChampSelectPosition {
    #[default]
    Top,
    Jungle,
    Middle,
    Bottom,
    Support,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum LolChampSelectRoles {
    #[default]
    Mage,
    Fighter,
    Tank,
    Assassin,
    Support,
    Marksman,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Path(String);

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
