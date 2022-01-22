use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use ulid::Ulid;

use juniper::GraphQLObject;

use tokio::sync::RwLock;

use crate::store::Store;
use crate::events::{self, EventStream, Event};


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
        _context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        todo!()
        /*
        let _store = context.read().await;
        let id = Ulid::new().to_string();
        return Command::success( {
            id,
            success: true,
            error: None,
        })
    */
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(context = RwLock<Context>)]
impl MutationRoot {
    async fn register(
        user_id: String,
        discord_username: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let id = Ulid::new();

        let event = events::types::ProfileRegistered {
            id,
            user_id: user_id.parse::<u64>().unwrap(),
            discord_username: discord_username.clone(),
        };

        process_event(context, event).await
    }

    async fn honor(
        to_user_id: String,
        by_user_id: String,
        amount: i32,
        reason: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let id = Ulid::new();
        let event = events::types::ComradeHonored {
            id,
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
            amount: amount as u64,
            reason: reason.clone(),
        };
        process_event(context, event).await
    }

    async fn dishonor(
        to_user_id: String,
        by_user_id: String,
        amount: i32,
        reason: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let id = Ulid::new();

        let event = events::types::ComradeDishonored {
            id,
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
            amount: amount as u64,
            reason: reason.clone(),
        };
        process_event(context, event).await
    }

    async fn jail(
        to_user_id: String,
        by_user_id: String,
        reason: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let id = Ulid::new();

        let event = events::types::ComradeJailed {
            id,
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
            reason: reason.clone(),
        };
        process_event(context, event).await
    }

    async fn unjail(
        to_user_id: String,
        by_user_id: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let event = events::types::ComradeUnjailed {
            id: Ulid::new(),
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
        };

        process_event(context, event).await
    }
}

async fn process_event<E: Event>(context: &RwLock<Context>, event: E) -> FieldResult<Command> {
    let mut context = context.write().await;

    match  event.validate(&context.store).await {
        Err(msg) => Command::failed(msg),
        Ok(()) => {
            context.event_stream.append(&event).await;
            event.exec(&mut context.store).await.unwrap();
            Command::succeeded(&event)
        },
    }
}

#[derive(GraphQLObject)]
pub struct Command {
    success: bool,
    error: Option<String>,
    event_id: Option<String>,
}

impl Command {
    fn failed<S: AsRef<str>>(error_message: S) -> FieldResult<Command> {
        return Ok(Command {
            success: false,
            error: Some(error_message.as_ref().to_string()),
            event_id: None,
        })
    }

    fn succeeded(event: &dyn Event) -> FieldResult<Command> {
        return Ok(Command {
            success: true,
            error: None,
            event_id: Some(event.id().to_string()),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<RwLock<Context>>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::new(),
    )
}
