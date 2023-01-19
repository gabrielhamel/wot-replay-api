use std::collections::HashMap;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

// Generated by https://transform.tools/json-to-rust-serde

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleInformation {
    pub player_vehicle: String,
    pub client_version_from_xml: String,
    pub map_display_name: String,
    pub client_version_from_exe: String,
    pub player_name: String,
    #[serde(rename = "playerID")]
    pub player_id: i64,
    pub server_name: String,
    pub vehicles: HashMap<String, VehicleResults>,
    pub server_settings: ServerSettings,
    pub date_time: String,
    pub map_name: String,
    #[serde(rename = "gameplayID")]
    pub gameplay_id: String,
    pub battle_type: i64,
    pub has_mods: bool,
    pub region_code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleResults {
    #[serde(rename = "personalMissionIDs")]
    pub personal_mission_ids: Vec<Value>,
    pub wtr: i64,
    pub vehicle_type: String,
    pub is_alive: i64,
    pub name: String,
    pub veh_post_progression: Vec<Value>,
    pub personal_mission_info: HashMap<String, Vec<i64>>,
    pub forbid_in_battle_invitations: bool,
    pub fake_name: String,
    pub max_health: i64,
    pub igr_type: i64,
    pub clan_abbrev: String,
    pub ranked: Vec<i64>,
    pub badges: Vec<Vec<i64>>,
    pub custom_role_slot_type_id: i64,
    pub team: i64,
    pub events: HashMap<String, Value>,
    pub overridden_badge: i64,
    #[serde(rename = "avatarSessionID")]
    pub avatar_session_id: String,
    pub is_team_killer: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerSettings {
    pub spg_redesign_features: SpgRedesignFeatures,
    #[serde(rename = "ranked_config")]
    pub ranked_config: RankedConfig,
    #[serde(rename = "battle_royale_config")]
    pub battle_royale_config: BattleRoyaleConfig,
    #[serde(rename = "comp7_config")]
    pub comp7_config: Value,
    pub roaming: (i64, i64, Vec<(i64, i64, i64, String)>, Vec<Value>),
    #[serde(rename = "epic_config")]
    pub epic_config: EpicConfig,
    #[serde(rename = "vehicle_post_progression_config")]
    pub vehicle_post_progression_config: VehiclePostProgressionConfig,
    pub is_potapov_quest_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpgRedesignFeatures {
    pub stun_enabled: bool,
    pub mark_target_area_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedConfig {
    pub rank_groups: Vec<i64>,
    pub year_rating_page_url: String,
    pub shop_state: String,
    pub arena_time: i64,
    pub unburnable_ranks: Vec<i64>,
    pub bonus_battles_multiplier: i64,
    pub max_players_in_queue: i64,
    pub year_reward_state: String,
    #[serde(rename = "minXP")]
    pub min_xp: i64,
    pub shop_page_url: String,
    pub num_players: i64,
    pub unburnable_step_ranks: Vec<Value>,
    pub min_level: i64,
    pub winner_rank_changes: Vec<i64>,
    #[serde(rename = "yearLBSize")]
    pub year_lbsize: i64,
    pub intro_page_url: String,
    pub forbidden_veh_types: Vec<i64>,
    pub imbalance: VehiclesTypeNumber,
    pub season_gap_page_url: String,
    pub seasons: HashMap<String, Season>,
    pub efficiency_groups: HashMap<String, Vec<f64>>,
    pub loser_rank_changes: Vec<i64>,
    pub max_vehicles: VehiclesTypeNumber,
    pub mm_fail_times: Vec<i64>,
    pub cycle_finish_seconds: i64,
    pub divisions: HashMap<String, Division>,
    pub has_special_season: bool,
    #[serde(rename = "yearLBState")]
    pub year_lbstate: String,
    pub qualification_battles: i64,
    pub archivate_after: i64,
    #[serde(rename = "peripheryIDs")]
    pub periphery_ids: Vec<i64>,
    pub prime_times: HashMap<String, PrimeTime>,
    pub acc_ranks: i64,
    pub max_level: i64,
    pub is_enabled: bool,
    pub info_page_url: String,
    pub qualification_bonus_battles: Vec<QualificationBonusBattle>,
    pub expected_seasons: i64,
    pub max_time_in_queue: i64,
    pub acc_steps: Vec<i64>,
    pub maps: Vec<Vec<i64>>,
    pub template: VehiclesTypeNumber,
    pub expire_seasons: i64,
    pub year_awards_marks: Vec<i64>,
    pub leagues_bonus_battles: Vec<LeaguesBonusBattle>,
    pub forbidden_class_tags: Vec<Value>,
    pub shields: HashMap<String, Vec<i64>>,
    pub cycle_times: Vec<Value>,
    pub season_rating_page_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cycle {
    pub end: i64,
    pub start: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub start_season: i64,
    pub end_season: i64,
    pub cycles: HashMap<String, Cycle>,
    pub number: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiclesTypeNumber {
    pub wheeled_vehicle: i64,
    pub heavy_tank: i64,
    #[serde(rename = "AT-SPG")]
    pub at_spg: i64,
    pub scout: i64,
    pub medium_tank: i64,
    #[serde(rename = "SPG")]
    pub spg: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Division {
    pub start_rank: i64,
    pub bonus_battles: Value,
    pub is_league: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BonusBattle {
    pub efficiency: f64,
    pub battles_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimeTime {
    pub start: Vec<i64>,
    pub end: Vec<i64>,
    pub weekdays: Vec<i64>,
    #[serde(rename = "peripheryIDs")]
    pub periphery_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualificationBonusBattle {
    pub steps: i64,
    pub battles_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaguesBonusBattle {
    pub steps: i64,
    pub battles_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleRoyaleConfig {
    pub spawn: HashMap<String, Spawn>,
    pub arena: Arena,
    pub cone_visibility: ConeVisibility,
    pub default_ammo: Vec<(String, Vec<f64>)>,
    #[serde(rename = "peripheryIDs")]
    pub periphery_ids: Vec<i64>,
    pub prime_times: HashMap<String, PrimeTime>,
    pub url: String,
    pub is_enabled: bool,
    pub economics: HashMap<String, Economic>,
    pub vehicles_slots_config: HashMap<String, VehiclesSlotsConfig>,
    #[serde(rename = "battleXP")]
    pub battle_xp: BattleXp,
    pub seasons: HashMap<String, Season>,
    pub in_battle_upgrades: InBattleUpgrades,
    pub cycle_times: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spawn {
    pub spawn_key_points_per_sector: i64,
    pub spawn_sectors_amount: i64,
    pub placement_strategies: Vec<String>,
    pub spawn_key_points_choose_time: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arena {
    #[serde(rename = "typeIDs")]
    pub type_ids: Vec<i64>,
    pub max_battle_duration: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArenaParams {
    pub max_teams_in_arena: i64,
    pub min_queue_size: Vec<Vec<i64>>,
    #[serde(rename = "enableAdvanced123Protection")]
    pub enable_advanced123protection: bool,
    pub max_players_in_team: i64,
    pub max_time_in_queue: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConeVisibility {
    pub turret_settings: TurretSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurretSettings {
    pub default: Default,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub angle: f64,
    pub time: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rent {
    pub currency: String,
    pub amount: i64,
    pub time: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Economic {
    pub rent: Rent,
    pub test_drive: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiclesSlotsConfig {
    pub charge3: i64,
    pub charge2: i64,
    pub charge1: i64,
    pub charge4: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleXp {
    pub xp_to_lvl: Vec<i64>,
    pub player: Player,
    pub loot: HashMap<String, Value>,
    pub bot: Bot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub xp_for_hit: i64,
    pub xp_for_kill: i64,
    pub xp_for_damage_coef: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    pub xp_for_hit: i64,
    pub xp_for_kill: i64,
    pub xp_for_damage_coef: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InBattleUpgrades {
    pub combating_cooldown: f64,
    pub settling_cooldown: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleRoleEquipment {
    #[serde(rename = "equipmentID")]
    pub equipment_id: i64,
    pub cost: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpicConfig {
    pub kick_locked_queue_size: i64,
    pub credits_max: i64,
    pub defeated_bonus_credits: i64,
    #[serde(rename = "arenaTypeIDs")]
    pub arena_type_ids: Vec<i64>,
    pub winner_bonus_credits: i64,
    pub url: String,
    pub credits_modifier: f64,
    pub kick_locked_fraction: f64,
    pub credits_min_threshold: i64,
    pub unlockable_in_battle_veh_levels: Vec<i64>,
    pub max_time_in_queue: i64,
    pub valid_vehicle_levels: Vec<i64>,
    pub maps: Vec<String>,
    pub max_queue_size: i64,
    pub auto_start: bool,
    #[serde(rename = "peripheryIDs")]
    pub periphery_ids: Vec<Value>,
    pub max_battles_for_same_team: f64,
    pub seasons: HashMap<String, Season>,
    pub epic_meta_game: EpicMetaGame,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpicMetaGame {
    pub max_combat_reserve_level: i64,
    pub rewards: Rewards,
    pub max_battle_duration: i64,
    pub meta_level: MetaLevel,
    pub slots: VehicleTypedArray,
    pub default_slots: DefaultSlots,
    pub skip_params_validation: i64,
    pub in_battle_reserves_by_rank: InBattleReservesByRank,
    pub default_reserves: VehicleTypedArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReserve {
    pub tags: Vec<String>,
    pub price: i64,
    pub levels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rewards {
    pub combat_reserves: HashMap<String, CombatReserve>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaLevel {
    pub fame_pts_to_progress: Vec<i64>,
    pub max_level: i64,
    pub ability_points_for_level: Vec<i64>,
    pub fame_pts_by_rank: HashMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultSlots {
    pub heavy_tank: i64,
    #[serde(rename = "AT-SPG")]
    pub at_spg: i64,
    pub medium_tank: i64,
    pub light_tank: i64,
    #[serde(rename = "SPG")]
    pub spg: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleTypedArray {
    pub heavy_tank: Vec<i64>,
    #[serde(rename = "AT-SPG")]
    pub at_spg: Vec<i64>,
    pub medium_tank: Vec<i64>,
    pub light_tank: Vec<i64>,
    #[serde(rename = "SPG")]
    pub spg: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleTypedNestedArray {
    pub heavy_tank: Vec<Vec<i64>>,
    #[serde(rename = "AT-SPG")]
    pub at_spg: Vec<Vec<i64>>,
    pub medium_tank: Vec<Vec<i64>>,
    pub light_tank: Vec<Vec<i64>>,
    #[serde(rename = "SPG")]
    pub spg: Vec<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InBattleReservesByRank {
    pub ammo_levels: VehicleTypedNestedArray,
    pub slot_actions: VehicleTypedNestedArray,
    pub slots_by_rank: VehicleTypedNestedArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiclePostProgressionConfig {
    pub forbidden_vehicles: Vec<i64>,
    pub is_post_progression_enabled: bool,
    pub enabled_rented_vehicles: Vec<i64>,
    pub enabled_features: Vec<String>,
}