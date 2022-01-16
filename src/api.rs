use serenity::model::id::UserId;

#[derive(Clone)]
#[non_exhaustive]
pub struct Api {
}

impl Api {
    pub fn new() -> Api {
        Api {
        }
    }

    pub fn register(
        &self,
        user_id: UserId,
        discord_name: String,
    ) {
        println!("Registering:");
        println!("{:?}", user_id);
        println!("{:?}", discord_name);
    }


    pub fn jail(
        &self,
        to_user_id: UserId,
        by_user_id: UserId,
        reason: String,
    ) {
        println!("Jailing:");
        println!("{:?}", to_user_id);
        println!("{:?}", by_user_id);
        println!("{:?}", reason);
    }

    pub fn unjail(
        &self,
        to_user_id: UserId,
        by_user_id: UserId,
    ) {
        println!("Unjailing:");
        println!("{:?}", to_user_id);
        println!("{:?}", by_user_id);
    }

    pub fn honor(
        &self,
        to_user_id: UserId,
        by_user_id: UserId,
        amount: i32,
        reason: String,
    ) {
        println!("Honoring:");
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
