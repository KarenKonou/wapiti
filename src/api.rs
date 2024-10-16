use std::future::Future;

use reqwest::{Client, Error, Response};

pub fn get_public_timeline(client: &Client, url: &str) -> impl Future<Output = Result<Response, Error>> {
    client
    .get(url)
    .header("Accept", "Activity+JSON")
    .send()
}