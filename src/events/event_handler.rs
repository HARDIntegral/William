use poise::serenity_prelude as serenity;
use serenity::utils::MessageBuilder;

use crate::util::*;

const ANON_CHANNEL_ID: u64 = 1240733827622637639;
const SELF_ID: u64 = 1239079000337420358;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("[INIT] Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.channel_id.get() == ANON_CHANNEL_ID
                && new_message.author.id.get() != SELF_ID {
                anon(new_message, ctx).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

async fn anon (new_message: &serenity::Message, ctx: &serenity::Context) -> Result<(), Error> {
    new_message.delete(&ctx.http).await?;
    let msg_content = new_message.content.clone();
    let return_msg = MessageBuilder::new()
        .push("[")
        .push(id_to_id(new_message.author.id.get()).to_string())
        .push("]: ")
        .push(msg_content)
        .build();
    new_message.channel_id.say(&ctx.http, return_msg).await?;

    for i in new_message.attachments.iter() {
        let return_msg = MessageBuilder::new()
            .push(i.url.clone())
            .build();
        new_message.channel_id.say(&ctx.http, return_msg).await?;
    }
    Ok(())
}

fn id_to_id(id: u64) -> u64 {
    return id % ((id as f64).sqrt() as u64);
}