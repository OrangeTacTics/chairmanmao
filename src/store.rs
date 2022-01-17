// This trait is required to use `try_next()` on the cursor
use mongodb::{bson::doc, Collection, Database};
use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

async fn connect_to_mongo() -> Database {
    use mongodb::{Client, options::ClientOptions};

    let mongo_host = std::env::var("MONGO_HOST").unwrap();

    let client_options = ClientOptions::parse(mongo_host).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db: Database = client.database("DailyMandarinThread");
    db
}

pub struct Store {
    db: Database,
    profiles_collection: Collection<Profile>,
}

impl Store {
    pub async fn new() -> Store {
        let db: Database = connect_to_mongo().await;
        let profiles_collection = db.collection::<Profile>("Profiles");

        Store {
            db,
            profiles_collection,
        }
    }

    pub fn db(&self) -> &Database {
        &self.db
    }

    pub async fn register(&self, user_id: u64, discord_username: String) {
        let display_name = discord_username.clone();
        let profile = Profile {
            _id: None,
            user_id,
            discord_username,

            created: bson::DateTime::now(),
            last_seen: bson::DateTime::now(),

            roles: Vec::new(),
            display_name,
            credit: 1000,
            yuan: 0,
            hanzi: Vec::new(),
            mined_words: Vec::new(),
            defected: false,
        };

        self.profiles_collection.insert_one(profile, None).await.unwrap();
    }

    pub async fn load_profile(&self, user_id: u64) -> Option<Profile> {
        let filter = doc! {
            "user_id": user_id as i64,
        };
        self.profiles_collection.find_one(filter, None).await.unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    _id: Option<ObjectId>,
    user_id: u64,
    discord_username: String,

    created: bson::DateTime,
    last_seen: bson::DateTime,

    roles: Vec<String>,
    display_name: String,
    credit: usize,
    yuan: usize,
    hanzi: Vec<String>,
    mined_words: Vec<String>,
    defected: bool,
}
