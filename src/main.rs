

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::sync::Arc;

mod commands;
mod events;
mod util;

mod global_state;
use global_state::TEMP;

async fn get_temp() {
    loop {
        let mut temp = TEMP.lock().await;
        *temp = 0;
        tokio::time::sleep(
            std::time::Duration::from_secs(24 * 60 * 60)
        ).await;
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    tokio::spawn(get_temp());
    
    // Get the discord token set in `Secrets.toml`
    let token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;
    // Set the default prefix for the bot
    let default_prefix = "$";

    let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(default_prefix.into()),
            ..Default::default()
        },
        commands: vec![
            commands::ping::ping(), 
            commands::age::age(),
            commands::susify::susify(),
            commands::desusify::desusify(),
            commands::poo::poo(),
            commands::clean::clean(),
        ],
        event_handler: |ctx, event, framework, data| {
            Box::pin(events::event_handler::event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
        tokio::spawn(typing(ctx.http.clone(), 1240733827622637639));   
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(util::Data {})
        })
    })
    .build();

    // Create a new instance of the Client, logging in as a bot.
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    // Start listening for events by starting a single shard
    Ok(client.into()) 
}

async fn typing(http: Arc<serenity::Http>, channel_id: u64) {
    let channel = serenity::ChannelId::from(channel_id);
    loop {
        if let Err(why) = channel.broadcast_typing(&http).await {
            println!("Error sending typing indicator: {:?}", why);
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}