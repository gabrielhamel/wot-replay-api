use juniper::{EmptyMutation, EmptySubscription, RootNode, FieldResult};
use wot_replay_parser::ReplayParser;
use crate::error::ReplayApiError;
use crate::input::battle_informations::BattleInformations;
use crate::output::Replay;

pub struct QueryRoot;

async fn retrieve_replay(url: String) -> Result<ReplayParser, ReplayApiError> {
    let raw = reqwest::get(url).await?.bytes().await?;
    let parser = ReplayParser::parse(Vec::from(raw))?;
    Ok(parser)
}

#[juniper::graphql_object]
impl QueryRoot {
    async fn replay(url: String) -> FieldResult<Replay> {
        let file = retrieve_replay(url).await?;
        let replay_metadata = file.replay_json_start()?;
        let replay: BattleInformations = serde_json::from_value(replay_metadata.to_owned())?;
        Ok(Replay::from(&replay))
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}