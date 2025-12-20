use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;
use urlencoding::encode;

/// Commands to setup, provision and deploy ingest servers.
#[poise::command(
    slash_command,
    subcommands("droplet", "dns", "links"),
    subcommand_required
)]
pub async fn srt(_ctx: Context<'_>) -> Result<(), Error> {
    // Parent stub of the srt commands; this will never execute.
    Ok(())
}

#[poise::command(slash_command)]
pub async fn droplet(_ctx: Context<'_>) -> Result<(), Error> {
    // Parent stub of the droplet commands; this will never execute.
    Ok(())
}

#[poise::command(slash_command)]
pub async fn dns(_ctx: Context<'_>) -> Result<(), Error> {
    // Parent stub of the DNS commands; this will never execute.
    Ok(())
}

/// Provides a tallylight.io link and SRT links for producers and observers.
#[poise::command(slash_command)]
pub async fn links(
    ctx: Context<'_>,
    #[description = "Average ping to Europe. Minimum value is 50ms, maximum 500ms."]
    #[min = 50_u32]
    #[max = 500_u32]
    ping: u32,
    #[description = "(Optional) Stream ID - a unique identifier. Default is your username."]
    stream_id: Option<String>,
    #[description = "(Optional) Tallylight.io room name. Default is your username."]
    tally_room_id: Option<String>,
    #[description = "(Optional) Whether or not to show the producer's read URI. Default is true (show it)."]
    show_producer_read_uri: Option<bool>,
) -> Result<(), Error> {
    let stream_id = &stream_id.unwrap_or(ctx.author().name.clone());
    let tally_room_id = &tally_room_id.unwrap_or(ctx.author().name.clone());
    let show_producer_read_uri = show_producer_read_uri.unwrap_or(true);

    let observer_link = format!(
        "```srt://ingest.luau.info:10000?pkt_size=1316&latency={}&streamid=#!::m=publish,r={}```",
        ping * 4000,
        encode(&stream_id)
    );

    let tally_link = format!(
        "[Click Here](https://v2.tallylight.io/room/{}/view)",
        encode(&tally_room_id)
    );

    let producer_link = format!(
        "```srt://ingest.luau.info:10000?pkt_size=1316&latency=200000&streamid=#!::m=request,r={}```",
        encode(&stream_id)
    );

    let mut embed = CreateEmbed::default()
        .title("Links")
        .description("tallylight.io link and SRT links for producers and observers.")
        .field("Tally Light", tally_link, false)
        .field("Observer", observer_link, true);

    if show_producer_read_uri {
        embed = embed.field("Producer", producer_link, true);
    }

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
