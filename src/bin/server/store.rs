// This trait is required to use `try_next()` on the cursor
use mongodb::{bson::doc, Database};
use serde::{Serialize, Deserialize};

async fn connect_to_mongo() -> Database {
    use mongodb::{Client, options::ClientOptions};

    let mongo_host = std::env::var("MONGO_HOST").unwrap();

    let client_options = ClientOptions::parse(mongo_host).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db: Database = client.database("DailyMandarinThread");
    db
}

pub struct Store {
    profiles_collection: mongodb::Collection<Profile>,
}

impl Store {
    pub async fn new() -> Store {
        let db: Database = connect_to_mongo().await;
        let profiles_collection = db.collection::<Profile>("Profiles");

        Store {
            profiles_collection,
        }
    }

    pub async fn profile_count(&mut self) -> mongodb::error::Result<u64> {
        self.profiles_collection.count_documents(None, None).await
    }

    pub async fn register(
        &mut self,
        user_id: u64,
        discord_username: String,
    ) -> mongodb::error::Result<Profile> {
        let count = self.profile_count().await?;
        let yuan = if count == 0 {
            10000
        } else {
            0
        };

        let display_name = discord_username.clone();
        let profile = Profile {
            user_id,
            discord_username,

            created: bson::DateTime::now(),
            last_seen: bson::DateTime::now(),

            roles: Vec::new(),
            display_name,
            credit: 1000,
            yuan,
            hanzi: Vec::new(),
            mined_words: Vec::new(),
            defected: false,
        };

        self.profiles_collection.insert_one(profile.clone(), None).await?;
        Ok(profile)
    }

    pub async fn load_profile(&self, user_id: u64) -> Option<Profile> {
        let filter = doc! {
            "user_id": user_id as i64,
        };
        self.profiles_collection.find_one(filter, None).await.unwrap()
    }

    pub async fn store_profile(&mut self, profile: &Profile) {
        let filter = doc! {
            "user_id": profile.user_id as i64,
        };
        self.profiles_collection.replace_one(filter, profile, None).await.unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub user_id: u64,
    pub discord_username: String,

    pub created: bson::DateTime,
    pub last_seen: bson::DateTime,

    pub roles: Vec<String>,
    pub display_name: String,
    pub credit: usize,
    pub yuan: usize,
    pub hanzi: Vec<String>,
    pub mined_words: Vec<String>,
    pub defected: bool,
}

impl Profile {
    pub fn add_role(&mut self, role: &str) -> bool {
        if !self.has_role(role) {
            self.roles.push(role.to_string());
            self.roles.sort();
            true
        } else {
            false
        }
    }

    pub fn remove_role(&mut self, role: &str) -> bool {
        if self.has_role(role) {
            let index = self.roles.iter().position(|x| x == &role.to_string()).unwrap();
            self.roles.remove(index);
            true
        } else {
            false
        }

    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
}
