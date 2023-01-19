use crate::input::battle_information::BattleInformation;
use crate::input::battle_results::BattlesResults;

pub struct ReplayInput {
    pub results: BattlesResults,
    pub information: BattleInformation
}

pub mod battle_information;
pub mod battle_results;