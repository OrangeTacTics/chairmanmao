use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use ulid::Ulid;

use juniper::GraphQLObject;

use chairmanmao::store::Store;


#[derive(Clone)]
pub struct Context {
    store: Store,
}

impl juniper::Context for Context {}

impl Context {
    pub async fn new() -> Context {
        let store = Store::new().await;
        Context {
            store,
        }
    }
}

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn ok() -> FieldResult<Event> {
        let id = Ulid::new().to_string();
        return Ok(Event {
            id,
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    async fn register(
        user_id: String,
        discord_username: String,
        context: &Context,
    ) -> FieldResult<Event> {
        let id = Ulid::new().to_string();

        println!("Registering");
        context.store.register(user_id.parse().unwrap(), discord_username).await;
        println!("Done");


        return Ok(Event {
            id,
        })
    }
}

#[derive(GraphQLObject)]
pub struct Event {
    id: String,
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::new(),
    )
}
