mod api;
mod app;
mod cache;
mod canteen_picker;
mod config;
mod meal_row;
mod model;

use relm4::RelmApp;
use relm4::gtk::prelude::*;

const APP_ID: &str = "de.jukromer.LunchTray";

fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install the TLS crypto provider");

    relm4::gtk::gio::resources_register_include!("lunch-tray.gresource")
        .expect("Failed to register resources");

    let app = RelmApp::new(APP_ID);
    relm4::main_application().set_resource_base_path(Some("/de/jukromer/LunchTray"));
    app.run::<app::App>(());
}
