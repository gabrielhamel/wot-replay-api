use crate::input::battle_informations::BattleInformations;
use crate::output::map::Map;
use crate::output::player::Player;
use crate::output::server::Server;
use crate::output::version::Version;

mod map;
mod version;
mod player;
mod server;

#[derive(GraphQLObject)]
pub struct Replay {
    pub date: String,
    pub player: Player,
    pub version: Version,
    pub map: Map,
    pub server: Server,
}

impl From<&BattleInformations> for Replay {
    fn from(value: &BattleInformations) -> Self {
        Replay {
            date: value.date_time.clone(),
            player: Player::from(value),
            map: Map::from(value),
            version: Version::from(value),
            server: Server::from(value)
        }
    }
}
