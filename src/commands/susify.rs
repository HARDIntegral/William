use poise::serenity_prelude as serenity;

use crate::util::*;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn susify(
    ctx: poise::Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), poise::Error> {
    let u = user.unwrap_or_else(|| ctx.author());
    
}