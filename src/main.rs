mod api;
mod app;
mod canteen_picker;
mod meal_row;
mod model;
mod config;
mod cache;

use relm4::RelmApp;

const APP_ID: &str = "de.jukromer.LunchTray";

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<app::App>(());
}
