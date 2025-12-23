use crate::{Context, Error};
use digitalocean_api::prelude::*;
use poise::CreateReply;
use serenity::all::CreateEmbed;

#[poise::command(slash_command, subcommands("update"), subcommand_required)]
pub async fn dns(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn update(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let mut embed = CreateEmbed::default();
    let cf_client = &ctx.data().cloudflare_client;
    let do_client = &ctx.data().digitalocean_client;
    let droplet = Droplet::list().execute(do_client).await?;
    let droplet = droplet.iter().find(|droplet| droplet.name() == "ingest");

    if let Some(droplet) = droplet {
        let ip_address = droplet
            .networks()
            .v4
            .iter()
            .find(|n| n.kind == "public")
            .map(|n| n.ip_address)
            .unwrap(); // TODO: Handle error.

        let response = cf_client.list_dns_records().await?;
        let records = response.result;
        let matched = records
            .iter()
            .find(|record| record.name == "ingest.luau.info");

        match matched {
            Some(_) => {
                cf_client.update_dns_record(ip_address).await?;
                embed = embed
                    .description("DNS records updated. They should propagate in about a minute.");
            }
            None => {
                cf_client.create_dns_record(ip_address).await?;
                embed = embed
                    .description("DNS records created. They should propagate in about a minute.");
            }
        }
    } else {
        embed = embed.description("No ingest server found. Try `/srt droplet create`.")
    }

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
