use std::collections::HashMap;
use wot_replay_parser::ReplayParser;
use crate::error::ReplayApiError;
use crate::input::battle_information::VehicleResults;
use crate::input::battle_results::{PlayerNameInfo, VehicleEarning};
use crate::input::battle_results::Vehicle as VehicleInfo;
use crate::input::ReplayInput;
use crate::output::score::Score;
use crate::output::vehicle::Vehicle;
use crate::scalars::u64::U64;

#[derive(GraphQLObject, Clone)]
pub struct Player {
    pub id: U64,
    pub name: String,
    pub anonymized_name: String,
    pub vehicle: Vehicle,
    pub is_anonymized: bool,
    pub score: Score,
}

pub fn from(parser: &ReplayParser, players_info: &HashMap<&String, &PlayerNameInfo>, vehicles_info: &HashMap<&String, &VehicleInfo>, value: &VehicleResults) -> Result<Player, ReplayApiError> {
    // Find player id
    let mut player_id = 0_u64;
    for player in players_info {
        if player.1.real_name == value.name {
            player_id = player.0.parse()?;
            break;
        }
        if player.1.name == value.name {
            player_id = player.0.parse()?;
            break;
        }
    }

    let mut vehicle_score: Option<&VehicleEarning> = None;
    for vehicle_section in vehicles_info {
        for vehicle_info in vehicle_section.1.iter() {
            if vehicle_info.account_dbid == player_id as i64 {
                vehicle_score = Some(vehicle_info);
                break;
            }
        }
    }

    Ok(Player {
        id: U64(player_id),
        name: value.name.clone(),
        anonymized_name: value.fake_name.clone(),
        is_anonymized: if value.name == value.fake_name { false } else { true },
        vehicle: Vehicle::parse(parser, value)?,
        score: Score::parse(vehicle_score.ok_or(ReplayApiError::ReplayJsonDecodeError)?)?,
    })
}

fn collect_users_info(input: &ReplayInput) -> (HashMap<&String, &PlayerNameInfo>, HashMap<&String, &VehicleInfo>) {
    let mut players_info: HashMap<&String, &PlayerNameInfo> = HashMap::new();
    let mut vehicles_info: HashMap<&String, &VehicleInfo> = HashMap::new();

    for result in &input.results {
        for x in &result.players {
            players_info.extend(x)
        }
        if result.vehicles == None {
            continue;
        }
        for x in &result.vehicles {
            vehicles_info.extend(x)
        }
    }
    (players_info, vehicles_info)
}

pub fn parse_players(parser: &ReplayParser, input: &ReplayInput) -> Result<Vec<Player>, ReplayApiError> {
    let users_info = collect_users_info(input);

    let mut result: Vec<Player> = vec![];
    for player in &input.information.vehicles {
        result.push(from(parser,&users_info.0, &users_info.1, player.1)?);
    }
    Ok(result)
}
