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

        let mut context = context.write().await;

        let event = Event::ProfileRegistered {
            id,
            user_id: user_id.parse::<u64>().unwrap(),
            discord_username: discord_username.clone(),
        };

        ////////////////
        // Validating //
        ////////////////
        let existing_profile = context.store.load_profile(user_id.parse().unwrap()).await;
        if existing_profile.is_some() {
            return Command::failed(format!("Profile with user_id already exists: {user_id}"));
        }

        ////////////////
        // Committing //
        ////////////////
        context.event_stream.append(&event).await;
        context.store.register(user_id.parse().unwrap(), discord_username).await;

        Command::succeeded(&event)
    }

    async fn honor(
        to_user_id: String,
        by_user_id: String,
        amount: i32,
        reason: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let id = Ulid::new();

        let mut context = context.write().await;

        let event = Event::ComradeHonored {
            id,
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
            amount: amount as u64,
            reason: reason.clone(),
        };

        ////////////////
        // Validating //
        ////////////////
        if amount <= 0 {
            return Command::failed("Amount must be positive".to_string());
        }

        let to_profile = context.store.load_profile(to_user_id.parse().unwrap()).await;
        if to_profile.is_none() {
            return Command::failed(format!("Not user exists with that toUserId: {}", &by_user_id));
        }

        let by_profile = context.store.load_profile(by_user_id.parse().unwrap()).await;
        if by_profile.is_none() {
            return Command::failed(format!("Not user exists with that byUserId: {}", &by_user_id));
        }

        let mut to_profile = to_profile.unwrap();

        if to_profile.user_id == by_profile.unwrap().user_id {
            return Command::failed("toUserId cannot be the same as fromUserId".to_string());
        }

        ////////////////
        // Committing //
        ////////////////
        context.event_stream.append(&event).await;
        to_profile.credit += amount as usize;
        context.store.store_profile(&to_profile).await;

        Command::succeeded(&event)
    }

    async fn dishonor(
        to_user_id: String,
        by_user_id: String,
        amount: i32,
        reason: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let id = Ulid::new();

        let mut context = context.write().await;

        let event = Event::ComradeDishonored {
            id,
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
            amount: amount as u64,
            reason: reason.clone(),
        };

        ////////////////
        // Validating //
        ////////////////
        if amount <= 0 {
            return Command::failed("Amount must be positive".to_string());
        }

        let to_profile = context.store.load_profile(to_user_id.parse().unwrap()).await;
        if to_profile.is_none() {
            return Command::failed(format!("Not user exists with that toUserId: {}", &by_user_id));
        }

        let by_profile = context.store.load_profile(by_user_id.parse().unwrap()).await;
        if by_profile.is_none() {
            return Command::failed(format!("Not user exists with that byUserId: {}", &by_user_id));
        }

        let mut to_profile = to_profile.unwrap();

        if to_profile.user_id == by_profile.unwrap().user_id {
            return Command::failed("toUserId cannot be the same as fromUserId".to_string());
        }

        if to_profile.credit < amount as usize {
            return Command::failed("User does not have enough remaining social credit".to_string());
        }

        ////////////////
        // Committing //
        ////////////////
        context.event_stream.append(&event).await;
        to_profile.credit -= amount as usize;
        context.store.store_profile(&to_profile).await;

        Command::succeeded(&event)
    }

    async fn jail(
        to_user_id: String,
        by_user_id: String,
        reason: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let id = Ulid::new();

        let mut context = context.write().await;

        let event = Event::ComradeJailed {
            id,
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
            reason: reason.clone(),
        };

        ////////////////
        // Validating //
        ////////////////
        let to_profile = context.store.load_profile(to_user_id.parse().unwrap()).await;
        if to_profile.is_none() {
            return Command::failed(format!("Not user exists with that toUserId: {}", &by_user_id));
        }

        let by_profile = context.store.load_profile(by_user_id.parse().unwrap()).await;
        if by_profile.is_none() {
            return Command::failed(format!("Not user exists with that byUserId: {}", &by_user_id));
        }

        let mut to_profile = to_profile.unwrap();

        if to_profile.user_id == by_profile.unwrap().user_id {
            return Command::failed("toUserId cannot be the same as fromUserId".to_string());
        }

        if to_profile.roles.contains(&"Jailed".to_string()) {
            return Command::failed("User is already jailed".to_string());
        }

        // if !by_profile.is_party() {
        //    return Command::failed("Jailer is not in the Party".to_string());
        // }

        ////////////////
        // Committing //
        ////////////////
        context.event_stream.append(&event).await;
        to_profile.roles.push("Jailed".to_string());
        to_profile.roles.sort();
        context.store.store_profile(&to_profile).await;

        Command::succeeded(&event)
    }

    async fn unjail(
        to_user_id: String,
        by_user_id: String,
        context: &RwLock<Context>,
    ) -> FieldResult<Command> {
        let id = Ulid::new();

        let mut context = context.write().await;

        let event = Event::ComradeUnjailed {
            id,
            to_user_id: to_user_id.parse::<u64>().unwrap(),
            by_user_id: by_user_id.parse::<u64>().unwrap(),
        };

        ////////////////
        // Validating //
        ////////////////
        let to_profile = context.store.load_profile(to_user_id.parse().unwrap()).await;
        if to_profile.is_none() {
            return Command::failed(format!("Not user exists with that toUserId: {}", &by_user_id));
        }

        let by_profile = context.store.load_profile(by_user_id.parse().unwrap()).await;
        if by_profile.is_none() {
            return Command::failed(format!("Not user exists with that byUserId: {}", &by_user_id));
        }

        let mut to_profile = to_profile.unwrap();

        if to_profile.user_id == by_profile.unwrap().user_id {
            return Command::failed("toUserId cannot be the same as fromUserId".to_string());
        }

        if !to_profile.roles.contains(&"Jailed".to_string()) {
            return Command::failed("User is not jailed".to_string());
        }

        // if !by_profile.is_party() {
        //    return Command::failed("Jailer is not in the Party".to_string());
        // }

        ////////////////
        // Committing //
        ////////////////
        context.event_stream.append(&event).await;
        let index = to_profile.roles.iter().position(|x| x == &"Jailed").unwrap();
        to_profile.roles.remove(index);
        context.store.store_profile(&to_profile).await;

        Command::succeeded(&event)
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

    fn succeeded(event: &Event) -> FieldResult<Command> {
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
