use std::env;
use std::str::FromStr;
use wot_extractor::WotExtractor;
use wot_replay_parser::{BattleEvent, ReplayParser};
use crate::error::ReplayApiError;
use crate::input::battle_information::VehicleResults;
use crate::output::nation::Nation;

#[derive(GraphQLObject, Clone)]
pub struct Position {
    time: f64,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(GraphQLObject, Clone)]
pub struct Vehicle {
    pub nation: Nation,
    pub name: String,
    pub display_name: String,
    pub positions: Vec<Position>,
}

fn get_position_packets(parser: &ReplayParser, vehicle_id: i32) -> Result<Vec<Position>, ReplayApiError> {
    let mut context = parser.context()?;
    let battle_start_offset = parser.battle_start_time();
    let mut res: Vec<Position> = vec![];
    for probable_packet in parser.packet_stream() {
        let packet = &probable_packet?;
        match BattleEvent::parse(packet, &mut context) {
            Ok(event) => {
                match event {
                    BattleEvent::Position(position) => {
                        if position.entity_id != vehicle_id {
                            continue;
                        }
                        if packet.time() - battle_start_offset < 0_f32 {
                            continue;
                        }
                        res.push(Position {
                            time: (packet.time() - battle_start_offset) as f64,
                            x: position.position.x as f64,
                            y: position.position.y as f64,
                            z: position.position.z as f64,
                        });
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }
    Ok(res)
}

impl Vehicle {
    pub fn parse(parser: &ReplayParser, value: &VehicleResults) -> Result<Vehicle, ReplayApiError> {
        let positions = get_position_packets(parser, i32::from_str(&value.avatar_session_id)?)?;
        let vehicle_type = value.vehicle_type.split(':').collect::<Vec<&str>>();
        let extractor = WotExtractor::from(&*env::var("WOT_PATH")?);
        let nation_locale = extractor.localization.nation(vehicle_type[0])?;
        let mut nation = Nation::from(&nation_locale);
        let tank = extractor.localization.tank(vehicle_type[0], vehicle_type[1])?;
        nation.name = String::from(vehicle_type[0]);
        Ok(Vehicle {
            nation,
            name: String::from(vehicle_type[1]),
            display_name: tank,
            positions
        })
    }
}