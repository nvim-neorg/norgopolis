use std::collections::HashMap;

use norgopolis_protos::client_communication::MessagePack;
use tokio;

#[derive(serde::Serialize)]
struct QueryArguments {
    path: String,
    query: String,
    num_jobs: Option<usize>,
}

#[derive(serde::Deserialize, Debug)]
struct ParseQueryResult {
    _file: String,
    _captures: HashMap<String, Vec<String>>,
}

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".into();
    let port = "62020".into();

    let mut handle = norgopolis_client::connect(&ip, &port).await.unwrap();

    // handle
    //     .invoke(
    //         "hello-world".into(),
    //         "echo".into(),
    //         Some(MessagePack {
    //             data: rmp_serde::to_vec("Hello!").unwrap(),
    //         }),
    //         |str: String| println!("{}", str),
    //     )
    //     .await
    //     .unwrap();

    handle
        .invoke(
            "norgopolis-breeze".into(),
            "parse-query".into(),
            Some(MessagePack {
                data: rmp_serde::to_vec(&QueryArguments {
                    path: "/home/vhyrro/neorg/".into(),
                    query: "(_) @value".into(),
                    num_jobs: None,
                })
                .unwrap(),
            }),
            |str: ParseQueryResult| println!("{:#?}", str),
        )
        .await
        .unwrap();
}
