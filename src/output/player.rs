use crate::input::battle_information::BattleInformation;

#[derive(GraphQLObject)]
pub struct Player {
    id: i32,
    name: String
}

impl From<&BattleInformation> for Player {
    fn from(value: &BattleInformation) -> Self {
        Player {
            id: value.player_id as i32,
            name: value.player_name.clone()
        }
    }
}
