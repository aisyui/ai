extern crate reqwest;

pub async fn post_request(refresh: String) -> String {

    let url = "https://bsky.social/xrpc/com.atproto.server.refreshSession";

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Authorization", "Bearer ".to_owned() + &refresh)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    return res
}
