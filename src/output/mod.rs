use crate::error::ReplayApiError;
use crate::input::ReplayInput;
use crate::output::map::Map;
use crate::output::server::Server;
use crate::output::version::Version;

mod map;
mod version;
mod player;
mod server;
mod vehicle;

#[derive(GraphQLObject)]
pub struct Replay {
    pub date: String,
    pub player_id: i32,
    pub version: Version,
    pub map: Map,
    pub server: Server,
    pub players: Vec<player::Player>
}

impl Replay {
    pub fn create(replay: ReplayInput) -> Result<Replay, ReplayApiError> {
        Ok(Replay {
            date: replay.information.date_time.clone(),
            player_id: replay.information.player_id as i32,
            map: Map::from(&replay.information),
            version: Version::from(&replay.information),
            server: Server::from(&replay.information),
            players: player::parse_players(&replay)?
        })
    }
}
