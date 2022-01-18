use ulid::Ulid;
use std::sync::Arc;
use serenity::model::id::UserId;
use redis::Client;
use redis::aio::Connection;
use redis::AsyncCommands;
use futures::lock::Mutex;

#[derive(Clone)]
#[non_exhaustive]
pub struct Api {
    connection: Arc<Mutex<Connection>>,
}

impl Api {
    pub async fn new() -> Api {
        let host = std::env::var("REDIS_HOST").unwrap().to_string();
        let client = Client::open(host.clone()).unwrap();
        let connection = Arc::new(Mutex::new(client.get_async_connection().await.unwrap()));

        Api {
            connection,
        }
    }

    pub async fn register(
        &self,
        user_id: UserId,
        discord_name: String,
    ) {
        let mut conn = self.connection.lock().await;
        let id = Ulid::new();
        let () = conn.xadd("events", "*", &[
            ("id", id.to_string()),
            ("type", "ProfileRegistered".to_string()),
            ("user_id", user_id.to_string()),
            ("discord_username", discord_name.to_string()),
        ]).await.unwrap();

        println!("Registering:");
        println!("{:?}", user_id);
        println!("{:?}", discord_name);
    }


    pub async fn jail(
        &self,
        to_user_id: UserId,
        by_user_id: UserId,
        reason: String,
    ) {
        let mut conn = self.connection.lock().await;
        let id = Ulid::new();
        let () = conn.xadd("events", "*", &[
            ("type", "ComradeJailed".to_string()),
            ("id", id.to_string()),
            ("to_user_id", to_user_id.to_string()),
            ("by_user_id", by_user_id.to_string()),
            ("reason", reason.to_string()),
        ]).await.unwrap();

        println!("Jailing:");
        println!("{:?}", to_user_id);
        println!("{:?}", by_user_id);
        println!("{:?}", reason);
    }

    pub async fn unjail(
        &self,
        to_user_id: UserId,
        by_user_id: UserId,
    ) {
        let mut conn = self.connection.lock().await;
        let id = Ulid::new();
        let () = conn.xadd("events", "*", &[
            ("type", "ComradeUnjailed".to_string()),
            ("id", id.to_string()),
            ("to_user_id", to_user_id.to_string()),
            ("by_user_id", by_user_id.to_string()),
        ]).await.unwrap();

        println!("Unjailing:");
        println!("{:?}", to_user_id);
        println!("{:?}", by_user_id);
    }

    pub async fn honor(
        &self,
        to_user_id: UserId,
        by_user_id: UserId,
        amount: i32,
        reason: String,
    ) {
        let mut conn = self.connection.lock().await;
        let id = Ulid::new();
        let () = conn.xadd("events", "*", &[
            ("type", "ComradeHonored".to_string()),
            ("id", id.to_string()),
            ("to_user_id", to_user_id.to_string()),
            ("by_user_id", by_user_id.to_string()),
            ("reason", reason.to_string()),
            ("amount", amount.to_string()),
        ]).await.unwrap();

        println!("Honoring:");
        println!("{:?}", to_user_id);
        println!("{:?}", by_user_id);
        println!("{:?}", amount);
        println!("{:?}", reason);
    }

    pub async fn dishonor(
        &self,
        to_user_id: UserId,
        by_user_id: UserId,
        amount: i32,
        reason: String,
    ) {
        let mut conn = self.connection.lock().await;
        let id = Ulid::new();
        let () = conn.xadd("events", "*", &[
            ("type", "ComradeDishonored".to_string()),
            ("id", id.to_string()),
            ("to_user_id", to_user_id.to_string()),
            ("by_user_id", by_user_id.to_string()),
            ("reason", reason.to_string()),
            ("amount", amount.to_string()),
        ]).await.unwrap();

        println!("Dishonoring:");
        println!("{:?}", to_user_id);
        println!("{:?}", by_user_id);
        println!("{:?}", amount);
        println!("{:?}", reason);
    }

    pub fn log_message(
        &self,
        by_user_id: UserId,
        message: String,
    ) {
        println!("Log message:");
        println!("{:?}", by_user_id);
        println!("{:?}", message);
    }
}
