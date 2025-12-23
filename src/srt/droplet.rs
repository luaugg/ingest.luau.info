use crate::{Context, Error};

use digitalocean_api::prelude::*;
use poise::CreateReply;
use serenity::all::CreateEmbed;

#[poise::command(slash_command, subcommands("create", "delete"), subcommand_required)]
pub async fn droplet(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, owners_only)]
pub async fn create(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    Droplet::create(
        "ingest",
        "lon1",
        "c-2",
        &ctx.data().digitalocean_snapshot_id,
    )
    .execute(&ctx.data().digitalocean_client)
    .await?;

    let embed = CreateEmbed::default()
        .description("Droplet created. Later, follow up this command with `/srt dns update`.");
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(slash_command, owners_only)]
pub async fn delete(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let mut embed = CreateEmbed::default();
    let droplet = Droplet::list()
        .execute(&ctx.data().digitalocean_client)
        .await?;

    let matching = droplet.iter().find(|droplet| droplet.name() == "ingest");
    if let Some(droplet) = matching {
        Droplet::delete(*droplet.id())
            .execute(&ctx.data().digitalocean_client)
            .await?;
        embed = embed.description("The droplet has been deleted.")
    } else {
        embed = embed.description("No droplet found with the name 'ingest'.")
    }

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
