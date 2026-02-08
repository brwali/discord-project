mod db;
use poise::serenity_prelude as serenity;
use std::env;

type Pool = db::DbPool;

struct Data {
    pool: Pool,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
async fn register_user(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let pool = ctx.data().pool.clone();
    let discord_id = ctx.author().id.to_string();
    match db::add_generic_user(pool, discord_id) {
        Ok(()) => {
            ctx.say("Successfully added user").await?;
        },
        Err(e) => {
            println!("Error {} occured adding user to DB", e);
            ctx.say("An Error occured when registering yourself to the database").await?;
        },
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // This is a basic way to determine if we are launching the bot for dev purposes.
    let args: Vec<String> = env::args().collect();
    let dev = args.iter().any(|arg| arg == "dev");
    dotenv::dotenv().ok();

    let db_path = if dev { "data.db" } else { "prod_data.db" };

    let token = if dev {
        env::var("DEV_DISCORD_TOKEN").expect("Expected a token in the environment")
    } else {
        env::var("DISCORD_TOKEN").expect("Expected a token in the environment")
    };
    let pool = db::init_pool(db_path).expect("Failed to create DB pool");
    let intents = serenity::GatewayIntents::all();
    let framework = poise::Framework
        ::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register_user()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            // clone the pool into the Data we return to command contexts
            let pool = pool.clone();
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool })
            })
        })
        .build();
    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;
    client.unwrap().start().await.unwrap();
}
