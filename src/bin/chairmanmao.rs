use dotenv;
use std::sync::Arc;
use std::sync::Mutex;
use std::env;
//use bytes::Bytes;

use chairmanmao::api;
use chairmanmao::command_parser;

use serde::{Serialize, Deserialize};

/*
use tokio::time::{
    sleep,
    Duration,
};
*/

use serenity::{
    async_trait,
    model::channel::Message,
    model::channel::Reaction,
    model::gateway::Ready,
//    model::guild::Role,
    model::id::*,
    prelude::*,
};

//use redis::Commands;


#[derive(Serialize, Deserialize, Debug)]
struct User {
    user_id: String,
    display_name: String,
    roles: Vec<String>,
}


struct Handler {
    redis: Redis,
    api: api::Api,
}

impl Handler {
    fn new(redis: Redis, api: api::Api) -> Handler {
        Handler { redis, api, }
    }

    async fn run_command(&self, _ctx: Context, msg: Message) -> Option<()> {
        let mut parser = command_parser::Parser::new(&msg.content);
        let command_name = parser.parse_command()?;

        println!("Command name: {}", &command_name);

        match command_name.as_str() {
            "register" => {
                let user_id = parser.parse_user_id()?;
                parser.end()?;
                self.api.register(user_id, "SDF".to_string());
            },
            "honor" => {
                let to_user_id = parser.parse_user_id()?;
                let by_user_id = msg.author.id;
                let amount = i32::try_from(parser.parse_integer()?).ok()?;
                let reason = parser.parse_rest();
                parser.end()?;
                self.api.honor(to_user_id, by_user_id, amount, reason);
            },
            "dishonor" => {
                let to_user_id = parser.parse_user_id()?;
                let by_user_id = msg.author.id;
                let amount = -i32::try_from(parser.parse_integer()?).ok()?;
                let reason = parser.parse_rest();
                parser.end()?;
                self.api.honor(to_user_id, by_user_id, amount, reason);
            },
            "jail" => {
                let to_user_id = parser.parse_user_id()?;
                let by_user_id = msg.author.id;
                let reason = parser.parse_rest();
                parser.end()?;
                self.api.jail(to_user_id, by_user_id, reason);
            },
            "unjail" => {
                let to_user_id = parser.parse_user_id()?;
                let by_user_id = msg.author.id;
                parser.end()?;
                self.api.unjail(to_user_id, by_user_id);
            },
            _ => return None,
        };


    //        &"!ping" => { msg.channel_id.say(&ctx.http, "Pong!").await.unwrap(); () }
        Some(())
    }
}

async fn reaction_users(ctx: Context, reaction: Reaction) -> Option<(UserId, UserId)> {
    let by_user_id = reaction.user_id?;
    let message = reaction.message(ctx).await.ok()?;
    let to_user_id = message.author.id;
    return Some((to_user_id, by_user_id));
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{:?}", msg);
        self.api.log_message(msg.author.id, msg.content.clone());

        if msg.content.starts_with("!") {
            self.run_command(ctx, msg).await;
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Some((to_user_id, by_user_id)) = reaction_users(ctx, reaction).await {
            if to_user_id != by_user_id {
                let amount = 1;
                let reason = "[REACTION]".to_owned();
                self.api.honor(to_user_id, by_user_id, amount, reason);
            }
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if let Some((to_user_id, by_user_id)) = reaction_users(ctx, reaction).await {
            if to_user_id != by_user_id {
                let amount = -1;
                let reason = "[REACTION]".to_owned();
                self.api.honor(to_user_id, by_user_id, amount, reason);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
        //println!("{:?}", ready.guilds);

        let guild_id = GuildId(env::var("GUILD_ID").unwrap().parse::<u64>().unwrap());

        let constants = chairmanmao::discord::DiscordConstants::load(&ctx.http, ready.user.id, guild_id).await;
        constants.tiananmen_channel.say(ctx, format!("Online {}", constants.mao_emoji)).await.unwrap();
//        tokio::spawn(background_loop(self.redis.clone()));
    }
}

async fn background_loop(redis: Redis) {
    let mut _conn = redis.connection.lock().unwrap();
    /*
    loop {
        println!("OK");
        {
            let mut conn = redis.connection.lock().unwrap();
            let users: Vec<u8> = conn.get("syncbot:users").unwrap();
            let users: String = String::from_utf8_lossy(&users).to_string();
            let users: Vec<User> = serde_json::from_str(&users).unwrap();
            println!("Desired user state: {:?}", users);
        }
        sleep(Duration::from_millis(10000)).await;
    }
    */
}

#[derive(Clone)]
struct Redis {
    connection: Arc<Mutex<redis::Connection>>,
}

impl Redis {
    fn new() -> Self {
        let host = env::var("REDIS_HOST").unwrap().to_string();
        let client = redis::Client::open(host.clone()).unwrap();
        let connection = Arc::new(Mutex::new(client.get_connection().unwrap()));
        Redis {
            connection,
        }
    }
}

/*
async fn download_emoji(emoji_id: &str) {
    let res = reqwest::get(format!("https://cdn.discordapp.com/emojis/{}.png", emoji_id))
        .await
        .unwrap();

    let headers = res.headers().clone();
    let data: Bytes = res.bytes().await.unwrap();

    println!("Headers {:?}", headers);
    println!("Name {:?}", &data);

    let mut outfile = std::fs::File::create("out.png").unwrap();
    let bytes: Vec<u8> = data.to_vec();
    copy(&mut std::io::Cursor::new(bytes), &mut outfile).unwrap();
}
*/


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
//    let emoji_id = "930693102266646598"; // eek
//    download_emoji(emoji_id).await;

    let token = env::var("DISCORD_TOKEN").unwrap();
    let redis = Redis::new();
    let api = api::Api::new();

    let mut client = Client::builder(&token)
        .event_handler(Handler::new(redis, api))
        .await
        .unwrap();

    client.start().await.unwrap();
}
