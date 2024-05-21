use crate::util::*;
use std::time::Instant;

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let msg = ctx.say("Calculating latency...").await?;
    let latency = start.elapsed().as_millis();

    msg.edit(
        ctx, poise::CreateReply::default()
            .content(format!("Pong! API latency is {}ms", latency))
        ).await?;

    Ok(())
}