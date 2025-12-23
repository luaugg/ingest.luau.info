use crate::{Context, Error};
use poise::CreateReply;
use serenity::all::CreateEmbed;

#[poise::command(slash_command, subcommands("update"), subcommand_required)]
pub async fn dns(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, owners_only)]
pub async fn update(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let cf_client = &ctx.data().cloudflare_client;
    let do_client = &ctx.data().digitalocean_client;

    let ip_addr = do_client.get_network_address().await?;
    let records = cf_client.list_dns_records().await?.result;
    let record = records.iter().find(|r| r.name == "ingest.luau.info");
    match record {
        Some(record) => cf_client.update_dns_record(&record.id, ip_addr).await?,
        None => cf_client.create_dns_record(ip_addr).await?,
    };

    let embed = CreateEmbed::default().description("DNS records updated.");
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
