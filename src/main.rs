mod api;
mod app;
mod cache;
mod canteen_picker;
mod config;
mod meal_row;
mod model;

use relm4::RelmApp;

const APP_ID: &str = "de.jukromer.LunchTray";

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<app::App>(());
}
