mod api;
mod app;
mod meal_row;
mod model;

use relm4::RelmApp;

const APP_ID: &str = "de.kromer.Lunchtray";

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<app::App>(());
}
