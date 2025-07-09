pub struct App {
    //screen we are displaying
    pub current_screen: CurrentScreen,
}

pub enum CurrentScreen {
    Main, //Main splash Screen
    Gameplay, //Main Gameplay Screen
    GameOver, //Screen When lost. NOTE: Maybe just a popup?
    About, //Screen about the product
}

impl App {
    //set up the game
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Main,
        }
    }
}
