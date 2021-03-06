use redis::AsyncCommands;
use ulid::Ulid;

use crate::store::Store;
use async_trait::async_trait;
use serde::{Serialize};

#[async_trait]
pub trait Event {
    fn id(&self) -> Ulid;

    fn type_name(&self) -> &'static str;

    async fn validate(&self, store: &Store) -> Result<(), String>;

    async fn exec(&self, store: &mut Store) -> Result<(), String>;

    fn to_map(&self) -> Vec<(String, String)>;
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

    pub async fn append<E: Event + Serialize>(&mut self, event: &E) {
        let map = vec![
            ("id".to_string(), event.id().to_string()),
            ("type".to_string(), event.type_name().to_string()),
            ("event".to_string(), serde_json::to_string(event).unwrap()),
        ];
        let () = self.redis.xadd("events", "*", &map).await.unwrap();
    }
}

pub mod types {
    use async_trait::async_trait;
    use super::Event;
    use crate::store::Store;
    use ulid::Ulid;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct ProfileRegistered {
         pub id: Ulid,
         pub user_id: u64,
         pub discord_username: String,
    }

    #[async_trait]
    impl Event for ProfileRegistered {
        fn id(&self) -> Ulid {
            self.id
        }

        fn type_name(&self) -> &'static str {
            "ProfileRegistered"
        }

        async fn validate(&self, store: &Store) -> Result<(), String> {
            let existing_profile = store.load_profile(self.user_id).await;
            if existing_profile.is_some() {
                return Err(format!("Profile with user_id already exists: {}", self.user_id));
            }
            Ok(())
        }

        async fn exec(&self, store: &mut Store) -> Result<(), String> {
            store.register(self.user_id, self.discord_username.clone()).await.unwrap();
            Ok(())
        }

        fn to_map(&self) -> Vec<(String, String)> {
            vec![
                ("id".to_string(), self.id().to_string()),
                ("type".to_string(), self.type_name().to_string()),
                ("user_id".to_string(), self.user_id.to_string()),
                ("discord_username".to_string(), self.discord_username.to_string()),
            ]
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct SetParty {
         pub id: Ulid,
         pub user_id: u64,
         pub flag: bool,
    }

    #[async_trait]
    impl Event for SetParty {
        fn id(&self) -> Ulid {
            self.id
        }

        fn type_name(&self) -> &'static str {
            "SetParty"
        }

        async fn validate(&self, store: &Store) -> Result<(), String> {
            let profile = store.load_profile(self.user_id).await;
            if profile.is_none() {
                return Err(format!("Not user exists with that id: {}", &self.user_id));
            }
            Ok(())
        }

        async fn exec(&self, store: &mut Store) -> Result<(), String> {
            let mut profile = store.load_profile(self.user_id).await.unwrap();
            if self.flag {
                profile.add_role("Party");
            } else {
                profile.remove_role("Party");
            }
            store.store_profile(&profile).await;
            Ok(())
        }

        fn to_map(&self) -> Vec<(String, String)> {
            vec![
                ("id".to_string(), self.id().to_string()),
                ("type".to_string(), self.type_name().to_string()),
                ("user_id".to_string(), self.user_id.to_string()),
                ("flag".to_string(), self.flag.to_string()),
            ]
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct ComradeHonored {
         pub id: Ulid,
         pub to_user_id: u64,
         pub by_user_id: u64,
         pub amount: u64,
         pub reason: String,
    }

    #[async_trait]
    impl Event for ComradeHonored {
        fn id(&self) -> Ulid {
            self.id
        }

        fn type_name(&self) -> &'static str {
            "ComradeHonored"
        }

        async fn validate(&self, store: &Store) -> Result<(), String> {
            if self.amount <= 0 {
                return Err("Amount must be positive".to_string());
            }

            let to_profile = store.load_profile(self.to_user_id).await;
            if to_profile.is_none() {
                return Err(format!("Not user exists with that toUserId: {}", &self.by_user_id));
            }

            let by_profile = store.load_profile(self.by_user_id).await;
            if by_profile.is_none() {
                return Err(format!("Not user exists with that byUserId: {}", &self.by_user_id));
            }

            if self.to_user_id == self.by_user_id {
                return Err("toUserId cannot be the same as fromUserId".to_string());
            }
            Ok(())
        }

        async fn exec(&self, store: &mut Store) -> Result<(), String> {
            let mut to_profile = store.load_profile(self.to_user_id).await.unwrap();
            to_profile.credit += self.amount as usize;
            store.store_profile(&to_profile).await;
            Ok(())
        }

        fn to_map(&self) -> Vec<(String, String)> {
            vec![
                ("id".to_string(), self.id().to_string()),
                ("type".to_string(), self.type_name().to_string()),
                ("to_user_id".to_string(), self.to_user_id.to_string()),
                ("by_user_id".to_string(), self.by_user_id.to_string()),
                ("amount".to_string(), self.amount.to_string()),
                ("reason".to_string(), self.reason.to_string()),
            ]
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct ComradeDishonored {
         pub id: Ulid,
         pub to_user_id: u64,
         pub by_user_id: u64,
         pub amount: u64,
         pub reason: String,
    }

    #[async_trait]
    impl Event for ComradeDishonored {
        fn id(&self) -> Ulid {
            self.id
        }

        fn type_name(&self) -> &'static str {
            "ComradeDishonored"
        }

        async fn validate(&self, store: &Store) -> Result<(), String> {
            if self.amount <= 0 {
                return Err("Amount must be positive".to_string());
            }

            let to_profile = store.load_profile(self.to_user_id).await;
            if to_profile.is_none() {
                return Err(format!("Not user exists with that toUserId: {}", &self.by_user_id));
            }

            let by_profile = store.load_profile(self.by_user_id).await;
            if by_profile.is_none() {
                return Err(format!("Not user exists with that byUserId: {}", &self.by_user_id));
            }

            if self.to_user_id == self.by_user_id {
                return Err("toUserId cannot be the same as fromUserId".to_string());
            }
            Ok(())
        }

        async fn exec(&self, store: &mut Store) -> Result<(), String> {
            let mut to_profile = store.load_profile(self.to_user_id).await.unwrap();
            to_profile.credit -= self.amount as usize;
            store.store_profile(&to_profile).await;
            Ok(())
        }

        fn to_map(&self) -> Vec<(String, String)> {
            vec![
                ("id".to_string(), self.id().to_string()),
                ("type".to_string(), self.type_name().to_string()),
                ("to_user_id".to_string(), self.to_user_id.to_string()),
                ("by_user_id".to_string(), self.by_user_id.to_string()),
                ("amount".to_string(), self.amount.to_string()),
                ("reason".to_string(), self.reason.to_string()),
            ]
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct ComradeJailed {
         pub id: Ulid,
         pub to_user_id: u64,
         pub by_user_id: u64,
         pub reason: String,
    }

    #[async_trait]
    impl Event for ComradeJailed {
        fn id(&self) -> Ulid {
            self.id
        }

        fn type_name(&self) -> &'static str {
            "ComradeJailed"
        }

        async fn validate(&self, store: &Store) -> Result<(), String> {
            let to_profile = store.load_profile(self.to_user_id).await;
            if to_profile.is_none() {
                return Err(format!("Not user exists with that toUserId: {}", &self.by_user_id));
            }

            let by_profile = store.load_profile(self.by_user_id).await;
            if by_profile.is_none() {
                return Err(format!("Not user exists with that byUserId: {}", &self.by_user_id));
            }

            let to_profile = to_profile.unwrap();

            if to_profile.user_id == by_profile.unwrap().user_id {
                return Err("toUserId cannot be the same as fromUserId".to_string());
            }

            if to_profile.roles.contains(&"Jailed".to_string()) {
                return Err("User is already jailed".to_string());
            }

            // if !by_profile.is_party() {
            //    return Command::failed("Jailer is not in the Party".to_string());
            // }
            Ok(())
        }

        async fn exec(&self, store: &mut Store) -> Result<(), String> {
            let mut to_profile = store.load_profile(self.to_user_id).await.unwrap();
            to_profile.roles.push("Jailed".to_string());
            to_profile.roles.sort();
            store.store_profile(&to_profile).await;
            Ok(())
        }

        fn to_map(&self) -> Vec<(String, String)> {
            vec![
                ("id".to_string(), self.id().to_string()),
                ("type".to_string(), self.type_name().to_string()),
                ("to_user_id".to_string(), self.to_user_id.to_string()),
                ("by_user_id".to_string(), self.by_user_id.to_string()),
                ("reason".to_string(), self.reason.to_string()),
            ]
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct ComradeUnjailed {
         pub id: Ulid,
         pub to_user_id: u64,
         pub by_user_id: u64,
    }

    #[async_trait]
    impl Event for ComradeUnjailed {
        fn id(&self) -> Ulid {
            self.id
        }

        fn type_name(&self) -> &'static str {
            "ComradeUnjailed"
        }

        async fn validate(&self, store: &Store) -> Result<(), String> {
            let to_profile = store.load_profile(self.to_user_id).await;
            if to_profile.is_none() {
                return Err(format!("Not user exists with that toUserId: {}", &self.by_user_id));
            }

            let by_profile = store.load_profile(self.by_user_id).await;
            if by_profile.is_none() {
                return Err(format!("Not user exists with that byUserId: {}", &self.by_user_id));
            }

            let to_profile = to_profile.unwrap();

            if to_profile.user_id == by_profile.unwrap().user_id {
                return Err("toUserId cannot be the same as fromUserId".to_string());
            }

            if !to_profile.roles.contains(&"Jailed".to_string()) {
                return Err("User is not jailed".to_string());
            }

            // if !by_profile.is_party() {
            //    return Err("Jailer is not in the Party".to_string());
            // }
            Ok(())
        }

        async fn exec(&self, store: &mut Store) -> Result<(), String> {
            let mut to_profile = store.load_profile(self.to_user_id).await.unwrap();
            let index = to_profile.roles.iter().position(|x| x == &"Jailed").unwrap();
            to_profile.roles.remove(index);
            store.store_profile(&to_profile).await;
            Ok(())
        }

        fn to_map(&self) -> Vec<(String, String)> {
            vec![
               ("id".to_string(), self.id().to_string()),
                ("type".to_string(), self.type_name().to_string()),
                ("to_user_id".to_string(), self.to_user_id.to_string()),
                ("by_user_id".to_string(), self.by_user_id.to_string()),
            ]
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct SetHsk {
         pub id: Ulid,
         pub user_id: u64,
         pub hsk: Option<u64>,
    }

    #[async_trait]
    impl Event for SetHsk {
        fn id(&self) -> Ulid {
            self.id
        }

        fn type_name(&self) -> &'static str {
            "SetHsk"
        }

        async fn validate(&self, store: &Store) -> Result<(), String> {
            let profile = store.load_profile(self.user_id).await;
            if profile.is_none() {
                return Err(format!("Not user exists with that user id: {}", &self.user_id));
            }

            if self.hsk.unwrap_or(0) > 6 {
                return Err(format!("Invalid HSK level: {}", self.hsk.unwrap_or(0)));
            }

            Ok(())
        }

        async fn exec(&self, store: &mut Store) -> Result<(), String> {
            let mut profile = store.load_profile(self.user_id).await.unwrap();
            profile.hsk = self.hsk;
            store.store_profile(&profile).await;
            Ok(())
        }

        fn to_map(&self) -> Vec<(String, String)> {
            vec![
               ("id".to_string(), self.id().to_string()),
                ("type".to_string(), self.type_name().to_string()),
                ("user_id".to_string(), self.user_id.to_string()),
                ("hsk".to_string(), self.hsk.map(|h| h.to_string()).unwrap_or("null".to_string())),
            ]
        }
    }
}
