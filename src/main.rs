use dotenv::dotenv;

use poise::serenity_prelude as serenity;

mod commands;
mod events;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Login with a bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
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
            commands::age::age()
        ],
        event_handler: |ctx, event, framework, data| {
            Box::pin(events::event_handler::event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(util::Data {})
        })
    })
    .build();

    // Create a new instance of the Client, logging in as a bot.
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    // Start listening for events by starting a single shard
    client.unwrap().start().await.unwrap();  
}
