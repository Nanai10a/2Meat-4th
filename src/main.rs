extern crate two_meat_rust;

use serenity::async_trait;
use serenity::client::ClientBuilder;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut client =
        match ClientBuilder::new(std::env::var("DISCORD_TOKEN").expect("Error: token not found!"))
            .guild_subscriptions(true)
            .event_handler(EventPoster)
            .await
        {
            Ok(client) => client,
            Err(err) => panic!("{}", err),
        };

    match client.start().await {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    }
}

struct EventPoster;

#[async_trait]
impl EventHandler for EventPoster {
    async fn message(&self, ctx: Context, message: Message) {
        if message.content == "!ping" {
            message.channel_id.say(ctx.http, "pong!").await;
        }
    }
}
