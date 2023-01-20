use wot_extractor::WotExtractor;
use crate::error::ReplayApiError;
use crate::input::battle_information::BattleInformation;

#[derive(GraphQLObject)]
pub struct Map {
    pub name: String,
    pub display_name: String
}

impl Map {
    pub fn parse(value: &BattleInformation) -> Result<Map, ReplayApiError> {
        let extractor = WotExtractor::from(option_env!("WOT_PATH").ok_or(ReplayApiError::BadConfigurationError)?);
        let map_locale = extractor.localization.map(&*value.map_name)?;
        Ok(Map {
            name: value.map_name.clone(),
            display_name: map_locale
        })
    }
}
