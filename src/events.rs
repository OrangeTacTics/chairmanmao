use serde_json::Value;

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub payload: EventPayload,
}


#[derive(Debug)]
pub enum EventPayload {
    ComradeHonored(payload::ComradeHonoredPayload),
    ProfileRegistered(payload::ProfileRegisteredPayload),
}

impl Event {
    pub fn load(bytes: &[u8]) -> Option<Event> {
//        let event = serde_json::from_slice::<Event>(payload);
        let value: Value = serde_json::from_slice::<Value>(bytes).ok()?;

        let id: String = value.get("id")?.as_str()?.to_owned();
        let type_name: &str = value.get("type")?.as_str()?;
        let payload_value: Value = value.get("payload")?.clone();
        println!("{} {}", id, type_name);
        println!("{}", payload_value);

        let payload = match type_name {
            "ComradeHonored" => {
                let payload = serde_json::from_value::<payload::ComradeHonoredPayload>(payload_value).unwrap();
                EventPayload::ComradeHonored(payload)
            },
            "ProfileRegistered" => {
                let payload = serde_json::from_value::<payload::ProfileRegisteredPayload>(payload_value).unwrap();
                EventPayload::ProfileRegistered(payload)
            },
            _ => panic!("OOK"),
        };
        Some(Event {
            id,
            payload,
        })
    }
}

mod payload {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ComradeHonoredPayload {
        pub to_user_id: String,
        pub by_user_id: String,
        pub amount: i32,
        pub reason: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ComradeJailedPayload {
        to_user_id: String,
        by_user_id: String,
        amount: i32,
        reason: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ProfileRegisteredPayload {
        user_id: String,
        discord_username: String,
    }
}

