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
            if 
                new_message.channel_id.get() == 1238638916119040065 ||
                new_message.channel_id.get() == 1238647629785862225 ||
                new_message.channel_id.get() == 1238647791954432072 ||
                new_message.channel_id.get() == 1238904709277024356 && 
                new_message.author.id.get() != SELF_ID 
            {
                log(new_message, ctx).await?;
            }
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let channel = serenity::ChannelId::new(1248877341917184112);
            let msg = MessageBuilder::new()
                .push("Welcome to the server, ")
                .push(new_member.user.name.clone())
                .push("!")
                .build();
            channel.say(&ctx.http, msg).await?;
            new_member.add_role(
                &ctx.http, serenity::model::id::RoleId::new(1238904839153651733)
            ).await?;
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

async fn log(new_message: &serenity::Message, ctx: &serenity::Context) -> Result<(), Error> {
    let log_channel = serenity::ChannelId::new(1248837889849032794);
    let mut log_msg = MessageBuilder::new();

    log_msg
        .push("User: ")
        .push(new_message.author.name.clone())
        .push(" [")
        .push(new_message.author.id.get().to_string())
        .push("] \n")
        .push("Channel: ")
        .push(new_message.channel_id.to_channel(&ctx.http).await?.guild().unwrap().name.clone())
        .push(" [")
        .push(new_message.channel_id.get().to_string())
        .push("] \n\n")
        .push("Message Content: \n\"")
        .push(new_message.content.clone())
        .push("\"\n");

    for i in new_message.attachments.iter() {
        log_msg.push("\n")
            .push(i.url.clone());
    }

    log_channel.say(&ctx.http, log_msg.push("\n").build()).await?;
        
    Ok(())
}

fn id_to_id(id: u64) -> u64 {
    return id % ((id as f64).sqrt() as u64);
}