use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;
use urlencoding::encode;

/// Generates syncer links with the provided options. \
/// Remember to press Show on the producer link.
///
/// syncer.live is created by Zusor. Check out their website at https://zusor.io/
#[poise::command(slash_command)]
pub async fn syncer(
    ctx: Context<'_>,
    #[description = "Label to show in syncer."] label: Option<String>,
    #[description = "Whether to show the system clock offset."] offset: Option<bool>,
    #[description = "Whether to center the syncer on the webpage."] center: Option<bool>,
    #[description = "Remote group ID of the syncer (so you can hide it later)."] group_id: Option<
        String,
    >,
    #[description = "Whether to split the syncer progress bar into segments."] split: Option<bool>,
    #[description = "The timezone to display the syncer in."] timezone: Option<String>,
    #[description = "Whether to enable extra precision on the second counter."]
    centiseconds: Option<bool>,
    #[description = "Whether to enable an audible beep each second."] beep: Option<bool>,
    #[description = "The frequency of the beep sound in Hz."]
    #[min = 200_u32]
    #[max = 1000_u32]
    frequency: Option<u32>,
) -> Result<(), Error> {
    let mut url = "```https://syncer.live/embed?".to_owned();

    url.push_str(&format!(
        "&label={}",
        encode(&label.unwrap_or(String::from("")))
    ));
    url.push_str(&format!("&offset={}", offset.unwrap_or(true)));
    url.push_str(&format!("&center={}", center.unwrap_or(true)));
    url.push_str(&format!(
        "&groupId={}",
        encode(&(&group_id).clone().unwrap_or(String::from("")))
    ));
    url.push_str(&format!("&split={}", split.unwrap_or(false)));
    url.push_str(&format!(
        "&timezone={}",
        encode(&timezone.unwrap_or(String::from("Etc/UTC")))
    ));
    url.push_str(&format!("&centiseconds={}", centiseconds.unwrap_or(false)));
    url.push_str(&format!("&beep={}", beep.unwrap_or(false)));
    url.push_str(&format!(
        "&beepFrequency={}```",
        frequency.unwrap_or(440_u32)
    ));

    let mut embed = CreateEmbed::default()
        .title("Syncer Links")
        .field("Observer", url, false);

    if let Some(group_id) = group_id {
        let producer_url = format!(
            "```https://syncer.live/control?groupId={}```",
            encode(&group_id)
        );
        embed = embed
            .description("If this is a new syncer, click **Show** on the Producer link to show it.")
            .field("Producer", producer_url, false);
    }

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
