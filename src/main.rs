mod camera;
mod cloudflare;
mod digitalocean;
mod srt;
mod syncer;

use crate::{cloudflare::CFClient, digitalocean::DOClient};
use poise::serenity_prelude as serenity;
use serenity::all::UserId;
use std::{collections::HashSet, env, hash::RandomState};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    cloudflare_client: CFClient,
    digitalocean_client: DOClient,
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
        cloudflare_client: CFClient::new(cloudflare_token, cloudflare_zone_id.clone()),
        digitalocean_client: DOClient::new(digitalocean_token, digitalocean_snapshot_id),
    };

    let owner_ids = vec![
        UserId::new(199217346911404032),
        UserId::new(283708023686299649),
        UserId::new(310424155910832130),
        UserId::new(816685888058687541),
        UserId::new(244234125194559488),
        UserId::new(181775064302223360),
        UserId::new(696258086511575071),
        UserId::new(422799538932154401),
        UserId::new(165206076990554113),
        UserId::new(213457568838713344),
        UserId::new(99186622326644736),
    ];

    let random_state = RandomState::new();
    let mut owners = HashSet::with_hasher(random_state);
    for id in owner_ids {
        owners.insert(id);
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![syncer::syncer(), srt::srt(), camera::cams()],
            owners: owners,
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
