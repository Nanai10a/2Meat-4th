use serenity::async_trait;
use serenity::client::ClientBuilder;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler};

pub async fn init() {
    let mut client =
        ClientBuilder::new(std::env::var("DISCORD_TOKEN").expect("Error: token not found!"))
            .guild_subscriptions(true)
            .event_handler(EventPoster)
            .await
            .unwrap();

    client.start().await.unwrap()
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
