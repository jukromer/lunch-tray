mod app;

use relm4::RelmApp;

const APP_ID: &str = "de.kromer.Lunchtray";

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<app::App>(());
}
