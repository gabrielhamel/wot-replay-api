use crate::input::battle_information::BattleInformation;

#[derive(GraphQLObject)]
pub struct Map {
    pub name: String,
    pub display_name: String
}

impl From<&BattleInformation> for Map {
    fn from(value: &BattleInformation) -> Self {
        Map {
            display_name: value.map_display_name.clone(),
            name: value.map_name.clone()
        }
    }
}
