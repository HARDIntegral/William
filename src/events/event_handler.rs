use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, CreateMessage, Message};
use poise::serenity_prelude::utils::MessageBuilder;
use poise::serenity_prelude::model::Colour;

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
                new_message.channel_id.get() == 1249807583070519367 ||
                new_message.channel_id.get() == 1238904709277024356 && 
                new_message.author.id.get() != SELF_ID 
            {
                msg_log(new_message, ctx, Colour::from_rgb(0, 255, 0)).await?;
            }
        }

        serenity::FullEvent::MessageDelete { channel_id, deleted_message_id, guild_id} => {
            let msg = ctx.cache.message(channel_id, deleted_message_id).map(|m| m.clone());

            if let Some(msg) = msg {
                if 
                msg.channel_id.get() == 1238638916119040065 ||
                msg.channel_id.get() == 1238647629785862225 ||
                msg.channel_id.get() == 1238647791954432072 ||
                msg.channel_id.get() == 1249807583070519367 ||
                msg.channel_id.get() == 1238904709277024356 && 
                msg.author.id.get() != SELF_ID {
                    msg_log(&msg, ctx, Colour::from_rgb(255, 0, 0)).await?;
                }
            } else {
                println!("Message not found in cache"); 
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

async fn msg_log(new_message: &serenity::Message, ctx: &serenity::Context, color: Colour) -> Result<(), Error> {
    let log_channel = serenity::ChannelId::new(1248837889849032794);

    let footer = CreateEmbedFooter::new("Timestamp:")
        .text(new_message.timestamp.to_string());
    let embed = CreateEmbed::new()
        .title(new_message.author.name.clone())
        .field(
            "Channel: ", 
            new_message
                .channel_id
                .to_channel(&ctx.http)
                .await?.guild()
                .unwrap()
                .name.clone(), 
            false
        )
        .field("Message Content:", new_message.content.clone(), false)
        .color(color) // Enclose the color value in square brackets
        .thumbnail(new_message.author.avatar_url().unwrap_or("".to_string()))
        .footer(footer);
    
    let builder = CreateMessage::new()
        .embed(embed);

    log_channel.send_message(&ctx.http, builder).await?;
        
    Ok(())
}

fn id_to_id(id: u64) -> u64 {
    return id % ((id as f64).sqrt() as u64);
}