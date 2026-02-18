use crate::{Context, Error};
use poise::{CreateReply, serenity_prelude as serenity};
use serenity::all::CreateEmbed;
use urlencoding::encode;

/// Generate camera links for a user.
#[poise::command(slash_command)]
pub async fn cams(
    ctx: Context<'_>,
    #[description = "The name of the user the camera link will be for."] name: String,
    #[description = "(Optional) Whether to use the ingest server for encoding camera feeds."]
    use_ingest: Option<bool>,
) -> Result<(), Error> {
    let ingest = use_ingest.unwrap_or(true);

    let send_uri = format!(
        "```https://vdo.ninja/?push={}{}```",
        encode(&name),
        if ingest {
            "&mediamtx=ingest.luau.info"
        } else {
            ""
        }
    );

    let view_uri = format!("```https://vdo.ninja/?view={}```", encode(&name));
    let reply = CreateReply::default();
    let embed = CreateEmbed::new()
        .title("Camera Links")
        .description("These links are for sending and receiving camera feeds.")
        .field("Caster", send_uri, false)
        .field("Producer", view_uri, false);

    ctx.send(reply.embed(embed)).await?;
    Ok(())
}
