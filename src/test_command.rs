use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn test_command(
    ctx: Context<'_>,
    #[description = "Echo this text"] text: String,
) -> Result<(), Error> {
    ctx.say(format!("You said: {}", text)).await?;
    Ok(())
}
