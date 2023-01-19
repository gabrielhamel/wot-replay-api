use crate::input::ReplayInput;
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

impl From<ReplayInput> for Replay {
    fn from(replay: ReplayInput) -> Self {
        Replay {
            date: replay.information.date_time.clone(),
            player: Player::from(&replay.information),
            map: Map::from(&replay.information),
            version: Version::from(&replay.information),
            server: Server::from(&replay.information)
        }
    }
}
