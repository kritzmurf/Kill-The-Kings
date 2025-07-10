use crate::ui;

#[derive(Default)]
pub struct App {
    //screen we are displaying
    current_screen: CurrentScreen,
    should_quit: bool,

}

impl App {
    //set up the game
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Main,
            should_quit: false,
        }
    }
}

#[derive(Default)]
pub enum CurrentScreen {
    #[default]
    Main, //Main splash Screen
    Gameplay, //Main Gameplay Screen
    GameOver, //Screen When lost. NOTE: Maybe just a popup?
    About, //Screen about the product
}
