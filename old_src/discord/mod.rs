use serenity::client::ClientBuilder;

mod impl_commands;
pub mod interface;
pub mod transceiver;
pub mod transferer;

pub async fn init(token: impl AsRef<str>) -> anyhow::Result<()> {
    let mut client = ClientBuilder::new(token)
        // .event_handler(DiscordInterface {})
        .await
        .unwrap();

    tokio::spawn(async move { client.start().await.unwrap() });
    Ok(())
}
