use std::convert::TryFrom;
use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use ulid::Ulid;
use serde::{Serialize};

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
    async fn profile(
        user_id: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Profile> {
        let context = context.read().await;
        let profile = context.store.load_profile(user_id.parse().unwrap()).await.unwrap();

        Ok(Profile {
            user_id: profile.user_id.to_string(),
            discord_username: profile.discord_username,
            display_name: profile.display_name,
            roles: profile.roles,
            credit: profile.credit as i32,
            yuan: profile.yuan as i32,
            created: profile.created.to_rfc3339_string(),
            hsk: profile.hsk.map(|h| h.try_into().unwrap()),
        })
    }
}

#[derive(GraphQLObject)]
pub struct Profile {
    pub user_id: String,
    pub discord_username: String,
    pub display_name: String,
    pub roles: Vec<String>,
    pub credit: i32,
    pub yuan: i32,
    pub created: String,
    pub hsk: Option<i32>,
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

    async fn set_party(
        user_id: String,
        flag: bool,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let event = events::types::SetParty {
            id: Ulid::new(),
            user_id: user_id.parse::<u64>().unwrap(),
            flag,
        };

        process_event(context, event).await
    }

    async fn set_hsk(
        user_id: String,
        hsk: Option<i32>,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let event = events::types::SetHsk {
            id: Ulid::new(),
            user_id: user_id.parse::<u64>().unwrap(),
            hsk: hsk.map(|h| u64::try_from(h).unwrap()),
        };

        process_event(context, event).await
    }
}

async fn process_event<E: Event + Serialize>(context: &RwLock<Context>, event: E) -> FieldResult<Command> {
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
