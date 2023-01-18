use crate::input::battle_informations::BattleInformations;

#[derive(GraphQLObject)]
pub struct Player {
    id: i32,
    name: String
}

impl From<&BattleInformations> for Player {
    fn from(value: &BattleInformations) -> Self {
        Player {
            id: value.player_id as i32,
            name: value.player_name.clone()
        }
    }
}
