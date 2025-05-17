use reqwest::{Client, Error };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    content: String,
    account: Account
}

#[derive(Deserialize)]
pub struct Account {
    display_name: String,
    username: String
}

pub async fn get_public_timeline(client: &Client, url: &str) -> Result<Vec<Post>, Error> {
    client
    .get(url.to_owned() + "/api/v1/timelines/public?local=true")
    .header("Accept", "Activity+JSON")
    .send()
    .await?
    .json::<Vec<Post>>()
    .await
}
