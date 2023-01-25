use juniper::{EmptyMutation, EmptySubscription, RootNode, FieldResult};
use wot_replay_parser::ReplayParser;
use crate::error::ReplayApiError;
use crate::input::ReplayInput;
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
        let json_start = file.replay_json_start()?;
        let json_end = file.replay_json_end().ok_or(ReplayApiError::ReplayJsonDecodeError)?;
        let replay_input = ReplayInput {
            information: serde_json::from_value(json_start.to_owned())?,
            results: serde_json::from_value(json_end.to_owned())?
        };
        Ok(Replay::create(&file, replay_input)?)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}