use crate::{Context, Error, cloudflare::*};
use cloudflare::endpoints::dns::dns::{DnsContent::A, ListDnsRecordsParams, UpdateDnsRecordParams};
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
    let cf_zone_id = ctx.data().cloudflare_zone_id.clone();
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

        let response = list_dns_records(
            cf_client,
            cf_zone_id.clone(),
            ListDnsRecordsParams::default(),
        )
        .await?;

        let records = response.result;
        let matched = records
            .iter()
            .find(|record| record.name == "ingest.luau.info");

        if let Some(record) = matched {
            let params = UpdateDnsRecordParams {
                ttl: Some(60),
                proxied: Some(false),
                name: "ingest.luau.info",
                content: A {
                    content: ip_address,
                },
            };

            update_dns_record(cf_client, cf_zone_id, record.id.clone(), params).await?;
            embed =
                embed.description("DNS records updated. They should propagate in about a minute.");
        } else {
            // TODO: Create the DNS record in this case.
            embed = embed.description("No matching DNS record found.");
        }
    } else {
        embed = embed.description("No ingest server found. Try `/srt droplet create`.")
    }

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
