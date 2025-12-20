pub mod dns;
pub mod droplet;
pub mod links;

use crate::{Context, Error};

/// Commands to setup, provision and deploy ingest servers.
#[poise::command(
    slash_command,
    subcommands("droplet::droplet", "dns::dns", "links::links"),
    subcommand_required
)]
pub async fn srt(_ctx: Context<'_>) -> Result<(), Error> {
    // Parent stub of the srt commands; this will never execute.
    Ok(())
}
