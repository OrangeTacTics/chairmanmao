use ulid::Ulid;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Event {
    pub id: Ulid,
    pub payload: EventPayload,
}


#[derive(Debug)]
pub enum EventPayload {
    ComradeHonored(payload::ComradeHonored),
    ProfileRegistered(payload::ProfileRegistered),
}

impl Event {
    pub fn from_map(map: &HashMap<String, String>) -> Option<Event> {
        let id = &map["id"];
        let event_type = &map["type"];

        let event = match event_type.as_str() {
            "ProfileRegistered" => payload::ProfileRegistered::from_map(id, map),
            "ComradeHonored" => payload::ComradeHonored::from_map(id, map),
            a => { println!("{}", a); None },
        };

        event
    }
}

mod payload {
    use std::collections::HashMap;
    use ulid::Ulid;
    use super::{Event, EventPayload};
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ComradeHonored {
        pub to_user_id: String,
        pub by_user_id: String,
        pub amount: i32,
        pub reason: String,
    }

    impl ComradeHonored {
        pub fn from_map(id: &str, map: &HashMap<String, String>) -> Option<Event> {
            let payload = EventPayload::ComradeHonored(ComradeHonored {
                to_user_id: map.get("to_user_id")?.to_string(),
                by_user_id: map.get("by_user_id")?.to_string(),
                amount: map.get("amount")?.parse::<i32>().ok()?,
                reason: map.get("reason")?.to_string(),
            });

            let id = Ulid::from_string(id).ok()?;

            Some(Event {
                id,
                payload,
            })
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ComradeJailed {
        to_user_id: String,
        by_user_id: String,
        amount: i32,
        reason: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ProfileRegistered {
        user_id: String,
        discord_username: String,
    }

    impl ProfileRegistered {
        pub fn from_map(id: &str, map: &HashMap<String, String>) -> Option<Event> {
            let payload = EventPayload::ProfileRegistered(ProfileRegistered {
                user_id: map.get("user_id")?.to_string(),
                discord_username: map.get("discord_username")?.to_string(),
            });

            let id = Ulid::from_string(id).ok()?;

            Some(Event {
                id,
                payload,
            })
        }
    }
}
