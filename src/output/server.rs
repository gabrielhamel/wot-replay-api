use crate::input::battle_information::BattleInformation;

#[derive(GraphQLObject)]
pub struct Server {
    name: String,
    region_code: String,
}

impl From<&BattleInformation> for Server {
    fn from(value: &BattleInformation) -> Self {
        Server {
            name: value.server_name.clone(),
            region_code: value.region_code.clone()
        }
    }
}
