use std::time::Duration;

use crate::{Context, Error};

use digitalocean_api::error::Error as DOError;
use poise::CreateReply;
use serenity::all::CreateEmbed;
use tokio::time::sleep;

#[poise::command(slash_command, subcommands("create", "delete"), subcommand_required)]
pub async fn droplet(_ctx: Context<'_>) -> Result<(), Error> {
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
            let embed = CreateEmbed::default().description(format!(
                "Droplet creation will finish in 45 seconds. ID: `{}`",
                droplet.id()
            ));

            let reply_builder = CreateReply::default().embed(embed.clone());
            let reply = ctx.send(reply_builder.clone()).await?;
            sleep(Duration::from_secs(45)).await;

            let ip_addr = do_client.get_network_address().await?;
            let records = cf_client.list_dns_records().await?.result;
            let record = records.iter().find(|r| r.name == "ingest.luau.info");
            let result = match record {
                Some(record) => cf_client.update_dns_record(&record.id, ip_addr).await,
                None => cf_client.create_dns_record(ip_addr).await,
            };

            let description = match result {
                Ok(_) => "Droplet is now ready. You can generate links with `/srt links`.",
                Err(err) => &format!("Failed to update DNS record: {}", err),
            };

            reply
                .edit(ctx, reply_builder.embed(embed.description(description)))
                .await?;
        }
        Err(err) => {
            let error_message = format!("Failed to create droplet: {}", err);
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
        Ok(_) | Err(DOError::NotFound) => "The droplet has been deleted.".to_owned(),
        Err(err) => format!("Failed to delete droplet: {}", err),
    };

    let embed = CreateEmbed::default().description(description);
    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
