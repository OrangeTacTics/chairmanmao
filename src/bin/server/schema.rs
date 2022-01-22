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
            success: true,
            error: None,
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

        let existing_profile = context.store.load_profile(user_id.parse().unwrap()).await;
        if existing_profile.is_some() {
            return Ok(GqlEvent {
                success: false,
                id: "".to_string(),
                error: Some(format!("Profile with user_id already exists: {user_id}")),
            });
        }

        context.event_stream.append(&event).await;
        context.store.register(user_id.parse().unwrap(), discord_username).await;
        println!("Done");

        return Ok(GqlEvent {
            success: true,
            id: id.to_string(),
            error: None,
        })
    }

    async fn honor(
        to_user_id: String,
        by_user_id: String,
        amount: i32,
        reason: String,
        context: &RwLock<Context>,
    ) -> FieldResult<GqlEvent> {
        assert!(amount > 0, "Amount must be positive");
        let id = Ulid::new();

        let mut context = context.write().await;

        println!("Honoring");
        let event = Event::ComradeHonored {
            id,
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
            amount: amount as u64,
            reason: reason.clone(),
        };

        let to_profile = context.store.load_profile(to_user_id.parse().unwrap()).await;
        if to_profile.is_none() {
            return Ok(GqlEvent {
                success: false,
                id: "".to_string(),
                error: Some(format!("Not user exists with that toUserId: {}", &by_user_id)),
            });
        }

        let by_profile = context.store.load_profile(by_user_id.parse().unwrap()).await;
        if by_profile.is_none() {
            return Ok(GqlEvent {
                success: false,
                id: "".to_string(),
                error: Some(format!("Not user exists with that byUserId: {}", &by_user_id)),
            });
        }

        let mut to_profile = to_profile.unwrap();

        context.event_stream.append(&event).await;

        to_profile.credit += amount as usize;
        context.store.store_profile(&to_profile).await;

        assert!(to_profile.user_id != by_profile.unwrap().user_id);

        println!("Done");

        return Ok(GqlEvent {
            success: true,
            id: id.to_string(),
            error: None,
        })
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "Event")]
pub struct GqlEvent {
    success: bool,
    error: Option<String>,
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
