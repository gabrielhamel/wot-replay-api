use wot_extractor::WotExtractor;
use crate::error::ReplayApiError;
use crate::input::battle_information::VehicleResults;
use crate::output::nation::Nation;

#[derive(GraphQLObject, Clone)]
pub struct Vehicle {
    pub nation: Nation,
    pub name: String,
    pub display_name: String,
}

impl Vehicle {
    pub fn parse(value: &VehicleResults) -> Result<Vehicle, ReplayApiError> {
        let vehicle_type = value.vehicle_type.split(':').collect::<Vec<&str>>();
        let extractor = WotExtractor::from(option_env!("WOT_PATH").ok_or(ReplayApiError::BadConfigurationError)?);
        let nation_locale = extractor.localization.nation(vehicle_type[0]).expect("mdr");
        let mut nation = Nation::from(&nation_locale);
        let tank = extractor.localization.tank(vehicle_type[0], vehicle_type[1])?;
        nation.name = String::from(vehicle_type[0]);
        Ok(Vehicle {
            nation,
            name: String::from(vehicle_type[1]),
            display_name: tank.name
        })
    }
}