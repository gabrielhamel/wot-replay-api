use crate::input::battle_information::BattleInformation;

#[derive(GraphQLObject)]
pub struct Map {
    pub name: String,
}

impl From<&BattleInformation> for Map {
    fn from(value: &BattleInformation) -> Self {
        Map {
            name: value.map_name.clone()
        }
    }
}
