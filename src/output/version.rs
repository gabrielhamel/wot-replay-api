use crate::input::battle_information::BattleInformation;

#[derive(GraphQLObject)]
pub struct Version {
    pub executable: String,
    pub xml: String
}

impl From<&BattleInformation> for Version {
    fn from(value: &BattleInformation) -> Self {
        Version {
            xml: value.client_version_from_xml.clone(),
            executable: value.client_version_from_exe.clone()
        }
    }
}
