pub mod links;

use crate::{Context, Error};
use digitalocean_api::error::Error as DOError;
use poise::CreateReply;
use serenity::all::CreateEmbed;
use std::time::Duration;
use tokio::time::sleep;

/// Commands to setup, provision and deploy ingest servers.
#[poise::command(
    slash_command,
    subcommands("create", "delete", "links::links"),
    subcommand_required
)]
pub async fn srt(_ctx: Context<'_>) -> Result<(), Error> {
    // Parent stub of the srt commands; this will never execute.
    Ok(())
}

/// Creates a new ingest server droplet. Takes about 45 seconds to complete.
#[poise::command(slash_command, owners_only)]
pub async fn create(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let cf_client = &ctx.data().cloudflare_client;
    let do_client = &ctx.data().digitalocean_client;

    match do_client.create_droplet().await {
        Ok(droplet) => {
            let mut embed = CreateEmbed::default().description(format!(
                "Server creation will finish in ~45 seconds. ID: `{}`",
                droplet.id()
            ));

            let reply = ctx
                .send(CreateReply::default().embed(embed.clone()))
                .await?;
            sleep(Duration::from_secs(45)).await;

            let ip_addr = do_client.get_network_address().await?;
            let records = cf_client.list_dns_records().await?.result;
            let record = records.iter().find(|r| r.name == "feed.rspwn.io");
            let result = match record {
                Some(record) => cf_client.update_dns_record(&record.id, ip_addr).await,
                None => cf_client.create_dns_record(ip_addr).await,
            };

            embed = match result {
                Ok(_) => embed
                    .description("Server is now ready. You can generate links with `/srt links`."),
                Err(err) => embed.description(&format!("Failed to update DNS record: {}", err)),
            };

            reply
                .edit(ctx, CreateReply::default().embed(embed.clone()))
                .await?;
        }
        Err(err) => {
            let error_message = format!("Failed to create server: {}", err);
            let embed = CreateEmbed::default().description(error_message);
            let reply = CreateReply::default().embed(embed);
            ctx.send(reply).await?;
        }
    }

    Ok(())
}

/// Deletes the ingest server droplet so that we don't incur charges.
#[poise::command(slash_command, owners_only)]
pub async fn delete(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let description = match ctx.data().digitalocean_client.delete_droplet().await {
        Ok(_) | Err(DOError::NotFound) => "The server has been deleted.".to_owned(),
        Err(err) => format!("Failed to delete server: {}", err),
    };

    let embed = CreateEmbed::default().description(description);
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
