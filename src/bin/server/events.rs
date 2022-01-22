use redis::AsyncCommands;
use ulid::Ulid;

pub enum Event {
    ProfileRegistered {
        id: Ulid,
        user_id: u64,
        discord_username: String,
    },
    ComradeHonored {
        id: Ulid,
        to_user_id: u64,
        by_user_id: u64,
        amount: u64,
        reason: String,
    },
    ComradeDishonored {
        id: Ulid,
        to_user_id: u64,
        by_user_id: u64,
        amount: u64,
        reason: String,
    },
    ComradeJailed {
        id: Ulid,
        to_user_id: u64,
        by_user_id: u64,
        reason: String,
    },
    ComradeUnjailed {
        id: Ulid,
        to_user_id: u64,
        by_user_id: u64,
    },
}

impl Event {
    pub fn id(&self) -> Ulid {
        use Event::*;
        match self {
            ProfileRegistered { id, user_id: _, discord_username: _ } => *id,
            ComradeHonored { id, to_user_id: _, by_user_id: _, amount: _, reason: _ } => *id,
            ComradeDishonored { id, to_user_id: _, by_user_id: _, amount: _, reason: _, } => *id,
            ComradeJailed { id, to_user_id: _, by_user_id: _, reason: _, } => *id,
            ComradeUnjailed { id, to_user_id: _, by_user_id: _, } => *id,
        }
    }

    pub fn type_name(&self) -> &'static str {
        use Event::*;
        match self {
            ProfileRegistered { id: _, user_id: _, discord_username: _ } => "ProfileRegistered",
            ComradeHonored { id: _, to_user_id: _, by_user_id: _, amount: _, reason: _ } => "ComradeHonored",
            ComradeDishonored { id: _, to_user_id: _, by_user_id: _, amount: _, reason: _, } => "ComradeDishonored",
            ComradeJailed { id: _, to_user_id: _, by_user_id: _, reason: _, } => "ComradeJailed",
            ComradeUnjailed { id: _, to_user_id: _, by_user_id: _, } => "ComradeUnjailed",
        }

    }

    fn to_map(&self) -> Vec<(String, String)> {
        let mut keys: Vec<(String, String)> = vec![
            ("id".to_string(), self.id().to_string()),
            ("type".to_string(), self.type_name().to_string()),
        ];
        use Event::*;
        match self {
            ProfileRegistered { id: _, user_id: _, discord_username: _, } => (),
            _ => (),
        }

        keys
    }
}

pub struct EventStream {
    redis: redis::aio::Connection,
}

impl EventStream {
    pub async fn new() -> EventStream {
        let host = std::env::var("REDIS_HOST").unwrap().to_string();

        let client = redis::Client::open(host.clone()).unwrap();
        let redis = client.get_async_connection().await.unwrap();

        EventStream {
            redis,
        }
    }

    pub async fn append(&mut self, event: &Event) {
        let () = self.redis.xadd("events", "*", &event.to_map()).await.unwrap();
    }
}
