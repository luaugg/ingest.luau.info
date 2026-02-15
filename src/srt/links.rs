use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;
use urlencoding::encode;

#[derive(Debug, poise::ChoiceParameter)]
pub enum TallyRoomChoice {
    #[name = "USS A Stream"]
    A,
    #[name = "USS B Stream"]
    B,
}

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
    #[description = "(Optional) Tallylight.io room name. USS A stream or B stream?"]
    tally_room_id: Option<TallyRoomChoice>,
) -> Result<(), Error> {
    let stream_id = &stream_id.unwrap_or(ctx.author().name.clone());
    let observer_ping = match region {
        "Europe" => 50,
        "US East" => 125,
        "US West" => 200,
        _ => 100,
    };

    let observer_link = format!(
        "```srt://ingest.luau.info:10000?pkt_size=1316&latency={}&streamid=publish:{}```",
        observer_ping * 4000,
        encode(&stream_id)
    );

    let tally_link = match tally_room_id {
        Some(TallyRoomChoice::A) => {
            Some("[Click Here](https://v2.tallylight.io/room/uss_spring_2026/view)".to_string())
        }
        Some(TallyRoomChoice::B) => Some(
            "[Click Here](https://v2.tallylight.io/room/uss_spring_2026_second/view)".to_string(),
        ),
        None => None,
    };

    let producer_ping = match producer_region {
        "Europe" => 50,
        "US East" => 125,
        "US West" => 200,
        _ => 100,
    };

    let producer_link = format!(
        "```srt://ingest.luau.info:10000?pkt_size=1316&latency={}&streamid=read:{}```",
        producer_ping * 4000,
        encode(&stream_id)
    );

    let mut embed = CreateEmbed::default()
        .title("Links")
        .description("tallylight.io link and SRT links for producers and observers.");

    if let Some(tally_link) = tally_link {
        embed = embed.field("Tally Light", tally_link, false);
    }

    embed = embed.field("Observer", observer_link, true);
    embed = embed.field("Producer", producer_link, true);

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
