use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use ulid::Ulid;

use juniper::GraphQLObject;

use tokio::sync::RwLock;

use crate::store::Store;
use crate::events::{EventStream, Event};


pub struct Context {
    store: Store,
    event_stream: EventStream,
}

impl juniper::Context for Context {}

impl Context {
    pub async fn new() -> Context {
        let store = Store::new().await;
        let event_stream = EventStream::new().await;
        Context {
            store,
            event_stream,
        }
    }
}

pub struct QueryRoot;

#[juniper::graphql_object(context = RwLock<Context>)]
impl QueryRoot {
    async fn ok(
        context: &RwLock<Context>,
    ) -> FieldResult<GqlEvent> {
        let _store = context.read().await;
        let id = Ulid::new().to_string();
        return Ok(GqlEvent {
            id,
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(context = RwLock<Context>)]
impl MutationRoot {
    async fn register(
        user_id: String,
        discord_username: String,
        context: &RwLock<Context>,
    ) -> FieldResult<GqlEvent> {
        let id = Ulid::new();

        let mut context = context.write().await;

        println!("Registering");
        let event = Event::ProfileRegistered {
            id,
            user_id: user_id.parse::<u64>().unwrap(),
            discord_username: discord_username.clone(),
        };

        context.event_stream.append(&event).await;
        context.store.register(user_id.parse().unwrap(), discord_username).await;
        println!("Done");

        return Ok(GqlEvent {
            id: id.to_string(),
        })
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "Event")]
pub struct GqlEvent {
    id: String,
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<RwLock<Context>>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::new(),
    )
}
