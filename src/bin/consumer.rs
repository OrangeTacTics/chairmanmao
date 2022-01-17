use std::collections::HashMap;
use chairmanmao::events::Event;

use redis::AsyncCommands;
use redis::streams::StreamRangeReply;
use redis::FromRedisValue;

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();

    let result: StreamRangeReply = con.xrange_all("events").await.unwrap();

    for event in result.ids.into_iter() {
        let map: HashMap<String, String> = event.map
            .into_iter()
            .map(|(k, v)| (k, String::from_redis_value(&v).unwrap()))
            .collect();

        let event = Event::from_map(&map);
        dbg!(event);

        println!();
        println!();
    }
}
