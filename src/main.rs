use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;
use std::env;

struct Handler {
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data: Ready) {
        
    }
}


async fn main() {
    // This is a basic way to determine if we are launching the bot for dev purposes.
    let args: Vec<String> = env::args().collect();
    let dev = args.iter().any(|arg| arg == "dev");
    println!("{}", dev);
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::all();
    let handler = Handler {};
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
