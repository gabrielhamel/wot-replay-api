use juniper::{EmptyMutation, EmptySubscription, RootNode, GraphQLInputObject};

pub struct QueryRoot;

#[derive(GraphQLObject)]
struct Replay {
    version: String,
    url: String
}

#[juniper::graphql_object]
impl QueryRoot {
    fn replay(url: String) -> Replay {
        Replay {
            version: String::from("1.74"),
            url
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}