use dotenv::dotenv;

use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

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
        commands: vec![ping(), age()],
        ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {})
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
