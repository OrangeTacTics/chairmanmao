use std::collections::HashMap;

use serenity::model::prelude::*;
use serenity::http::client::Http;

#[derive(Debug, Clone)]
pub struct DiscordConstants {
    pub guild_id: GuildId,
    pub bot_user_id: UserId,

    // ROLES
    pub comrade_role: Role,
    pub party_role: Role,
//    pub jailed_role: Role,
//    pub bumpers_role: Role,

    // CHANNELS
    // NEWS
//    pub news_channel: GuildChannel,
//    pub rules_channel: GuildChannel,
    // GENERAL
//    pub general_channel: GuildChannel,
//    pub exam_channel: GuildChannel,
//    pub learners_channel: GuildChannel,
//    pub apologies_channel: GuildChannel,
//    pub voice_channel: GuildChannel,
    // SPECIAL
//    pub party_channel: GuildChannel,
//    pub art_channel: GuildChannel,
//    pub bump_channel: GuildChannel,
    pub tiananmen_channel: GuildChannel,

    // EMOJIS
    pub mao_emoji: Emoji,
}

impl DiscordConstants {
    pub async fn load(http: &Http, bot_user_id: UserId, guild_id: GuildId) -> DiscordConstants {
        let GuildId(guild_id) = guild_id;
        let guild = http.get_guild(guild_id).await.unwrap();

        let channels: Vec<GuildChannel> = http.get_channels(guild_id).await.unwrap();
        for channel in channels.iter() {
            println!("{} {}", channel.id, channel.name);
        }

        let comrade_role = find_role(&guild.roles, "同志");
        let party_role = find_role(&guild.roles, "共产党员");
//        let jailed_role = find_role(&guild.roles, "劳改");
//        let learner_role = find_role(&guild.roles, "中文学习者");
//        let bumpers_role = find_role(&guild.roles, "Bumpers");

//        let news_channel = find_channel(&channels, "📰");
//        let rules_channel = find_channel(&channels, "🈲");
//        let thread_channel = find_channel(&channels, "🧵");
//        let commentators_channel = find_channel(&channels, "🐉");
//        let learners_channel = find_channel(&channels, "✍");
//        let exam_channel = find_channel(&channels, "🏫");
//        let apologies_channel = find_channel(&channels, "⛔");
        let tiananmen_channel = find_channel(&channels, "🏯");
//        let bump_channel = find_channel(&channels, "✊");

        println!("{:?}", &tiananmen_channel);

//        let eek_emoji = find_emoji(&guild.emojis, "eek");
        let mao_emoji = find_emoji(&guild.emojis, "mao");
//        let dekinai_emoji = find_emoji(&guild.emojis, "buneng");
//        let dekinai2_emoji = find_emoji(&guild.emojis, "buneng2");
//        let diesofcringe_emoji = find_emoji(&guild.emojis, "diesofcringe");
//        let rightist_emoji = find_emoji(&guild.emojis, "rightist");
//        let refold_emoji = find_emoji(&guild.emojis, "refold");

        DiscordConstants {
            guild_id: GuildId(guild_id),
            bot_user_id,
            comrade_role,
            party_role,
            tiananmen_channel,
            mao_emoji,
        }
    }
}

fn find_channel(channels: &[GuildChannel], name: &str) -> GuildChannel {
    for channel in channels {
        if channel.name.contains(name) {
            return channel.clone();
        }
    }
    panic!("Channel does not exist: {}", name);
}

fn find_role(roles: &HashMap<RoleId, Role>, name: &str) -> Role {
    for role in roles.values() {
        println!("{:?} vs {:?}", role.name, name);
        if role.name.contains(name) {
            return role.clone();
        }
    }
    panic!("Role does not exist: {}", name);

}

fn find_emoji(emojis: &HashMap<EmojiId, Emoji>, name: &str) -> Emoji {
    for emoji in emojis.values() {
        if emoji.name.contains(name) {
            return emoji.clone();
        }
    }
    panic!("Emoji does not exist: {}", name);
}
