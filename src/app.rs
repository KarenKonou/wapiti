pub struct App {
    pub instance_url: String,
    pub current_screen: CurrentScreen,
}

pub enum CurrentScreen {
    Login,
    Main,
}

impl App {
    pub fn new() -> App {
        App {
            instance_url: String::new(),
            current_screen: CurrentScreen::Main,
        }
    }
}
