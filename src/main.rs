use poise::serenity_prelude as serenity;
use std::env;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
async fn register_league(
    ctx: Context<'_>,
    #[description = "Full Id - name#tag"] riot_id: String,
) -> Result<(), Error> {
    let Some((name, tag)) = riot_id.split_once('#') else {
        // If no tag is given, send a message and then bail
        ctx.say("Riot ID must include a `#tag`").await?;
        return Ok(());
    };
    // For now just print the name and tag but we need to store this in the db
    // but before then we have to design the schema and what we want to store
    println!("{} {}", name, tag);
    Ok(())
}

#[tokio::main]
async fn main() {
    // This is a basic way to determine if we are launching the bot for dev purposes.
    let args: Vec<String> = env::args().collect();
    let dev = args.iter().any(|arg| arg == "dev");
    dotenv::dotenv().ok();

    let token = if dev {
        env::var("DEV_DISCORD_TOKEN").expect("Expected a token in the environment")
    } else {
        env::var("DISCORD_TOKEN").expect("Expected a token in the environment")
    };
    let intents = serenity::GatewayIntents::all();
    let framework = poise::Framework
        ::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register_league()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;
    client.unwrap().start().await.unwrap();
}
