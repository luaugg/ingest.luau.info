use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;

/// Generates syncer links with the provided options. \
/// Remember to press Show on the producer link.
///
/// syncer.live is created by Zusor. Check out their website at https://zusor.io/
#[poise::command(slash_command)]
pub async fn syncer(
    ctx: Context<'_>,
    #[description = "Which stream to use."]
    #[choices("USS A Stream", "USS B Stream")]
    stream: &'static str,
    #[description = "Whether to show the system clock offset."] offset: Option<bool>,
    #[description = "Whether to center the syncer on the webpage."] center: Option<bool>,
) -> Result<(), Error> {
    let mut url = "```https://syncer.live/embed?".to_owned();
    let mut producer_url = "```https://syncer.live/control?groupId=".to_owned();

    match stream {
        "USS A Stream" => {
            url.push_str("&label=USS%20A%20Stream");
            url.push_str("&groupId=uss-2026-summer-a");
            producer_url.push_str("uss-2026-summer-a```");
        }
        "USS B Stream" => {
            url.push_str("&label=USS%20B%20Stream");
            url.push_str("&groupId=uss-2026-summer-b");
            producer_url.push_str("uss-2026-summer-b```");
        }
        _ => (),
    }

    url.push_str(&format!("&offset={}", offset.unwrap_or(true)));
    url.push_str(&format!("&center={}```", center.unwrap_or(true)));

    let embed = CreateEmbed::default()
        .title("Syncer Links")
        .description("The **Producer** link is to control the syncer visibility.")
        .field("Observer", url, false)
        .field("Producer", producer_url, false);

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
