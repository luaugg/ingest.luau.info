use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;
use urlencoding::encode;

#[poise::command(slash_command, subcommands("check", "update"), subcommand_required)]
pub async fn dns(_ctx: Context<'_>) -> Result<(), Error> {
    // Parent stub of the DNS commands; this will never execute.
    Ok(())
}

#[poise::command(slash_command)]
pub async fn check(_ctx: Context<'_>) -> Result<(), Error> {
    // Implementation of the DNS check command.
    Ok(())
}

#[poise::command(slash_command)]
pub async fn update(_ctx: Context<'_>) -> Result<(), Error> {
    // Implementation of the DNS update command.
    Ok(())
}
