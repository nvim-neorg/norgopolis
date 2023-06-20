use norgopolis_client::communication::MessagePack;
use tokio;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".into();
    let port = "62020".into();

    let mut handle = norgopolis_client::connect(&ip, &port).await.unwrap();

    handle
        .invoke(
            "hello-world".into(),
            "echo".into(),
            Some(MessagePack {
                data: rmp_serde::to_vec("Hello!").unwrap(),
            }),
            |str: String| println!("{}", str),
        )
        .await
        .unwrap();
}
