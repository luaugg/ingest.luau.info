mod cloudflare;
mod srt;
mod syncer;

use ::cloudflare::framework::client::async_api::Client as CloudflareClient;
use digitalocean_api::prelude::*;
use poise::serenity_prelude as serenity;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    cloudflare_client: CloudflareClient,
    cloudflare_zone_id: String,
    digitalocean_client: DigitalOcean,
    digitalocean_snapshot_id: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let discord_token = env::var("DISCORD_BOT_TOKEN").expect("missing DISCORD_BOT_TOKEN env var");
    let cloudflare_token =
        env::var("CLOUDFLARE_API_TOKEN").expect("missing CLOUDFLARE_API_TOKEN env var");
    let cloudflare_zone_id =
        env::var("CLOUDFLARE_ZONE_ID").expect("missing CLOUDFLARE_ZONE_ID env var");
    let digitalocean_token =
        env::var("DIGITALOCEAN_API_TOKEN").expect("missing DIGITALOCEAN_API_TOKEN env var");
    let digitalocean_snapshot_id =
        env::var("DIGITALOCEAN_SNAPSHOT_ID").expect("missing DIGITALOCEAN_SNAPSHOT_ID env var");
    let data = Data {
        cloudflare_client: cloudflare::get_cloudflare_client(cloudflare_token)
            .await
            .expect("failed to create cloudflare client"),
        cloudflare_zone_id,
        digitalocean_client: DigitalOcean::new(digitalocean_token).unwrap(),
        digitalocean_snapshot_id,
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![syncer::syncer(), srt::srt()],
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        poise::FrameworkError::ArgumentParse { error, .. } => {
                            eprintln!("argument parse error: {:?}", error);
                        }
                        other => poise::builtins::on_error(other).await.unwrap(),
                    }
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let mut client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .expect("error creating bot client");

    client.start().await.expect("error running bot client");
}
