extern crate reqwest;
use std::collections::HashMap;

pub async fn post_request(handle: String, pass: String) -> String {

    let url = "https://bsky.social/xrpc/com.atproto.server.createSession";

    let mut map = HashMap::new();
    map.insert("identifier", &handle);
    map.insert("password", &pass);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    return res
}
