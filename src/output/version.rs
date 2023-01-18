use crate::input::battle_informations::BattleInformations;

#[derive(GraphQLObject)]
pub struct Version {
    pub executable: String,
    pub xml: String
}

impl From<&BattleInformations> for Version {
    fn from(value: &BattleInformations) -> Self {
        Version {
            xml: value.client_version_from_xml.clone(),
            executable: value.client_version_from_exe.clone()
        }
    }
}
