use crate::input::battle_informations::BattleInformations;

#[derive(GraphQLObject)]
pub struct Server {
    name: String,
    region_code: String,
}

impl From<&BattleInformations> for Server {
    fn from(value: &BattleInformations) -> Self {
        Server {
            name: value.server_name.clone(),
            region_code: value.region_code.clone()
        }
    }
}
