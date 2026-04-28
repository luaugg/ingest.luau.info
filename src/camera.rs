use crate::{Context, Error};
use poise::{CreateReply, serenity_prelude as serenity};
use serenity::all::CreateEmbed;
use urlencoding::encode;

/// Generate camera links for a user.
#[poise::command(slash_command)]
pub async fn cams(
    ctx: Context<'_>,
    #[description = "The name of the user the camera link will be for."] name: String,
) -> Result<(), Error> {
    let send_uri = format!("[Click me](https://vdo.ninja/?push={})", encode(&name),);

    let view_uri = format!("```https://vdo.ninja/?view={}```", encode(&name));
    let reply = CreateReply::default();
    let embed = CreateEmbed::new()
        .title("Camera Links")
        .description("These links are for sending and receiving camera feeds.")
        .field("Caster", send_uri, true)
        .field("Producer", view_uri, true);

    ctx.send(reply.embed(embed)).await?;
    Ok(())
}
