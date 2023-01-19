use std::collections::HashMap;
use crate::error::ReplayApiError;
use crate::input::battle_information::VehicleResults;
use crate::input::battle_results::PlayerNameInfo;
use crate::input::ReplayInput;
use crate::output::vehicle::Vehicle;

#[derive(GraphQLObject, Clone)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub vehicle: Vehicle
}

pub fn from(input: &HashMap<&String, &PlayerNameInfo>, value: &VehicleResults) -> Result<Player, ReplayApiError> {
    // Find player id
    let mut player_id = 0_i32;
    for player in input {
        if player.1.name == value.name {
            player_id = player.0.parse()?;
            break;
        }
    }

    Ok(Player {
        id: player_id,
        name: value.name.clone(),
        vehicle: Vehicle::from(value),
    })
}

fn collect_users_info(input: &ReplayInput) -> HashMap<&String, &PlayerNameInfo> {
    let mut map: HashMap<&String, &PlayerNameInfo> = HashMap::new();

    for result in &input.results {
        for x in &result.players {
            map.extend(x)
        }
    }
    map
}

pub fn parse_players(input: &ReplayInput) -> Result<Vec<Player>, ReplayApiError> {
    let users_info = collect_users_info(input);

    let mut result: Vec<Player> = vec![];
    for player in &input.information.vehicles {
        result.push(from(&users_info, player.1)?);
    }
    Ok(result)
}
