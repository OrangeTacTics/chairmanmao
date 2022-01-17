use serenity::prelude::*;
use serenity::model::prelude::*;
//use serenity::builder::CreateMessage;
use crate::exams::{Exam, ExamScore};

pub async fn comrade_honored(
    ctx: &Context,
    channel_id: ChannelId,
    amount: u32,
) -> Result<Message, SerenityError> {
    channel_id.send_message(&ctx, |m| {
        m.add_embed(|e| {
            e
                .title("Comrade has been honored!")
                .description(format!("Comrade {} has been granted {} social credit.", "???????", amount))
                .color(0xFF0000u32)
                .thumbnail("https://cdn.discordapp.com/emojis/889997514403618846.webp?size=96&quality=lossless")
                .author(|a| {
                    a
                        .name("???")
    //                    .icon_url("comrade.avatar_url")
                })

        })
    }).await
}

pub async fn comrade_dishonored(
    ctx: &Context,
    channel_id: ChannelId,
    amount: u32,
) -> Result<Message, SerenityError> {
    channel_id.send_message(&ctx, |m| {
        m.add_embed(|e| {
            e
                .title("Comrade has been dishonored!")
                .description(format!("Comrade {} has lost {} social credit.", "???????", amount))
                .color(0xFF0000u32)
                .thumbnail("https://cdn.discordapp.com/emojis/889997514403618846.webp?size=96&quality=lossless")
                .author(|a| {
                    a
                        .name("???")
    //                    .icon_url("comrade.avatar_url")
                })

        })
    }).await
}

/*
pub async fn comrade_jailed(
    ctx: &Context,
    channel_id: ChannelId,
    amount: u32,
) -> Result<Message, SerenityError> {
    channel_id.send_message(&ctx, |m| {
        m.add_embed(|e| {
            e
                .title("Comrade has been dishonored!")
                .description(format!("Comrade {} has lost {} social credit.", "???????", amount))
                .color(0xff0000u32)
                .thumbnail("https://cdn.discordapp.com/emojis/889997514403618846.webp?size=96&quality=lossless")
                .author(|a| {
                    a
                        .name("???")
    //                    .icon_url("comrade.avatar_url")
                })

        })
    }).await
}
*/

pub async fn exam_start(
    ctx: &Context,
    channel_id: ChannelId,
    exam: &Exam,
) -> Result<Message, SerenityError> {
    channel_id.send_message(&ctx, |m| {
        m.add_embed(|e| {
            e.color(0xFFA500u32);
            e.field("Exam", &exam.name, true);
            e.field("Questions", exam.num_questions.to_string(), true);
            let timelimit = format!("{} seconds", (exam.timelimit / 1000));
            e.field("Time Limit", timelimit, false);

            if let Some(max_wrong) = exam.max_wrong {
                e.field("Mistakes allowed", max_wrong.to_string(), true);
            }

//                .title("Exam is beginning")
//                .description(format!("Comrade {} has lost {} social credit.", "???????", amount))
//                .author(|a| {
//                    a
//                        .name("???")
//    //                    .icon_url("comrade.avatar_url")
//                })

            e
        })
    }).await
}

pub async fn exam_results(
    ctx: &Context,
    channel_id: ChannelId,
    score: &ExamScore,
) -> Result<Message, SerenityError> {
    let mut lines = Vec::<String>::new();

    for (question, answer) in score.graded_questions.iter() {
        let correct = answer.is_correct();
        let emoji = if correct { "✅" } else { "❌" };
        let _correct_answer = question.valid_answers[0].clone();
        let question_str = question.question.to_string(); // ljust(longest_answer + 2 "  ");
        // answer_str = answer if correct else f"{answer} → {correct_answer}"
        let answer_str = format!("{:?}", answer);
        let line = format!(
            "{}　{} {}　*{}*",
            emoji,
            question_str,
            answer_str,
            question.meaning,
        );
        lines.push(line);
    }

    channel_id.send_message(&ctx, |m| {
        m.add_embed(|e| {
            e.description(lines.join("\n"))
        })
    }).await
}
