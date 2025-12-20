use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::CreateEmbed;
use urlencoding::encode;

#[poise::command(slash_command)]
pub async fn tally(
    ctx: Context<'_>,
    #[description = "The name of the tallylight.io room."] room: String,
    #[description = "The port OBS WebSocket Server is listening on."]
    #[min = 1024]
    #[max = 65535]
    port: Option<u16>,
    #[description = "The password for the OBS WebSocket Server."] password: Option<String>,
) -> Result<(), Error> {
    let port = &port.unwrap_or(4455).to_string();
    let password = &password.unwrap_or(String::new().to_string());

    let transmitter_url = format!(
        "https://v2.tallylight.io/room/{}/transmit/obs?obsPort={}&obsPassword={}&mode=obs",
        encode(&room),
        encode(port),
        encode(password)
    );

    let tally_url = format!("https://v2.tallylight.io/room/{}/view", encode(&room));

    let embed = CreateEmbed::new()
        .title("Tally Light Links")
        .description("Add the transmitter URL to OBS and give the viewer URL to your observers.")
        .field(
            "Transmitter URL",
            format!("```{}```", transmitter_url),
            false,
        )
        .field("Viewer URL", format!("```{}```", tally_url), false);

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
