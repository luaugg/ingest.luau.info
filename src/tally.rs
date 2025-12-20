use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;
use urlencoding::encode;

#[poise::command(slash_command)]
pub async fn tally(
    ctx: Context<'_>,
    #[description = "The name of the tallylight.io room."] room: String,
) -> Result<(), Error> {
    let tally_setup_url = format!("https://v2.tallylight.io/room/{}/setup/obs", encode(&room));
    let tally_url = format!("https://v2.tallylight.io/room/{}/view", encode(&room));

    let embed = CreateEmbed::new()
        .title("Tally Light Links")
        .description("Click the setup URL to add the transmitter. The Viewer URL is for observers.")
        .field(
            "Setup URL",
            format!("[Click Here]({})", tally_setup_url),
            true,
        )
        .field("Viewer URL", format!("```{}```", tally_url), true);

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
