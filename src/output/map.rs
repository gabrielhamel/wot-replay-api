use crate::input::battle_informations::BattleInformations;

#[derive(GraphQLObject)]
pub struct Map {
    pub name: String,
    pub display_name: String
}

impl From<&BattleInformations> for Map {
    fn from(value: &BattleInformations) -> Self {
        Map {
            display_name: value.map_display_name.clone(),
            name: value.map_name.clone()
        }
    }
}
