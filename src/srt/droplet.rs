use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;
use urlencoding::encode;

#[poise::command(
    slash_command,
    subcommands("create", "delete", "status"),
    subcommand_required
)]
pub async fn droplet(_ctx: Context<'_>) -> Result<(), Error> {
    // Parent stub of the droplet commands; this will never execute.
    Ok(())
}

#[poise::command(slash_command)]
pub async fn create(_ctx: Context<'_>) -> Result<(), Error> {
    // Implementation for creating a droplet
    Ok(())
}

#[poise::command(slash_command)]
pub async fn delete(_ctx: Context<'_>) -> Result<(), Error> {
    // Implementation for deleting a droplet
    Ok(())
}

#[poise::command(slash_command)]
pub async fn status(_ctx: Context<'_>) -> Result<(), Error> {
    // Implementation for getting status of a droplet
    Ok(())
}
