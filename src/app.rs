use reqwest::Client;

use crate::api::{get_public_timeline, Post};

pub struct App {
    pub instance_url: String,
    pub current_screen: CurrentScreen,
    client: Client,
    pub vertical_scroll: usize,
    pub posts: Vec<Post>
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Login,
    Main,
}

impl App {
    pub fn new(client: Client) -> App {
        App {
            instance_url: String::new(),
            current_screen: CurrentScreen::Main,
            client,
            vertical_scroll: 0,
            posts: Vec::new()
        }
    }

    pub async fn get_public_timeline(&mut self) {
        if let Ok(res) = get_public_timeline(&self.client, &self.instance_url).await {
            self.posts = res
        }
    }
}
