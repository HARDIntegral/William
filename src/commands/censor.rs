use poise::serenity_prelude as serenity;

use crate::util::*;

#[poise::command(prefix_command, slash_command)]
pub async fn censor(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    if !ctx.author().has_role(
        ctx.http(), 
        ctx.guild_id().unwrap(), 
        serenity::model::id::RoleId::new(ADMIN_ID)).await? 
    {
        ctx.reply("You are not allowed to use this command").await?;
        return Ok(());
    }

    let user = user.ok_or("You must provide a user to censor")?;

    let member = ctx.guild_id().ok_or("")?.member(ctx.http(), user.id).await?;
    member.add_role(
        ctx, serenity::model::id::RoleId::new(1245954189868400670)
    ).await?;
    ctx.reply("User has their tongue cut out!").await?;
    Ok(())
}