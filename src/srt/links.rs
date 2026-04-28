use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;
use urlencoding::encode;

/// Provides a tallylight.io link and SRT links for producers and observers.
#[poise::command(slash_command)]
pub async fn links(
    ctx: Context<'_>,
    #[description = "Region choice - which region is the observer closest to?"]
    #[choices("Europe", "US East", "US West")]
    region: &'static str,
    #[description = "Region choice - which region is the producer closest to?"]
    #[choices("Europe", "US East", "US West")]
    producer_region: &'static str,
    #[description = "(Optional) Stream ID - a unique identifier. Default is your username."]
    stream_id: Option<String>,
) -> Result<(), Error> {
    let stream_id = &stream_id.unwrap_or(ctx.author().name.clone());
    let observer_ping = match region {
        "Europe" => 75,
        "US East" => 150,
        "US West" => 200,
        _ => 100,
    };

    let observer_link = format!(
        "```srt://feed.rspwn.io:10000?pkt_size=1316&latency={}&streamid=publish:{}```",
        observer_ping * 4000,
        encode(&stream_id)
    );

    let producer_ping = match producer_region {
        "Europe" => 75,
        "US East" => 150,
        "US West" => 200,
        _ => 100,
    };

    let producer_link = format!(
        "```srt://feed.rspwn.io:10000?pkt_size=1316&latency={}&streamid=read:{}```",
        producer_ping * 4000,
        encode(&stream_id)
    );

    let embed = CreateEmbed::default()
        .title("Links")
        .description("tallylight.io link and SRT links for producers and observers.")
        .field("Observer", observer_link, true)
        .field("Producer", producer_link, true);

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
