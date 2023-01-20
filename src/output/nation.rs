use wot_extractor::localization::NationLocale;

#[derive(GraphQLObject, Clone)]
pub struct Nation {
    pub name: String,
    pub display_name: String,
    pub nationality: String
}

impl From<&NationLocale> for Nation {
    fn from(value: &NationLocale) -> Self {
        Nation {
            name: value.name.clone(),
            display_name: value.name.clone(),
            nationality: value.nationality.clone()
        }
    }
}
