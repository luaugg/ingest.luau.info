mod syncer;

use std::env;

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![syncer::syncer()],
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
                Ok(Data {})
            })
        })
        .build();

    let token = env::var("DISCORD_BOT_TOKEN").expect("missing DISCORD_BOT_TOKEN env var");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("error creating bot client");

    client.start().await.expect("error running bot client");
}
