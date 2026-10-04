use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::api;
use crate::model::Day;

//ID of THA for early testing prupose
const CANTEEN_ID: u32 = 803;

pub struct App {
    text: String,
}

#[derive(Debug)]
pub enum CommandMsg {
    Loaded(Result<Vec<Day>, reqwest::Error>),
}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = CommandMsg;

    view! {
        adw::ApplicationWindow {
            set_title: Some("Lunch Tray"),
            set_default_size: (420, 640),

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    gtk::Label {
                        #[watch]
                        set_label: &model.text,
                        set_wrap: true,
                        set_xalign: 0.0,
                        set_valign: gtk::Align::Start,
                        set_margin_all: 12,
                    },
                },
            },
        }
    }
    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            text: String::from("Loading..."),
        };
        let widgets = view_output!();

        sender.oneshot_command(async { CommandMsg::Loaded(api::fetch_meals(CANTEEN_ID).await) });

        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            CommandMsg::Loaded(Ok(days)) => {
                self.text = describe(&days);
            }
            CommandMsg::Loaded(Err(error)) => {
                self.text = format!("Could not load the menu: {error}");
            }
        }
    }
}

fn describe(days: &[Day]) -> String {
    let mut text = String::new();
    for day in days {
        text.push_str(&format!("{}\n", day.date));
        for meal in &day.meals {
            text.push_str(&format!(
                "{} | {} | {:?}\n",
                meal.name, meal.category, meal.prices.students
            ));
        }
        text.push('\n');
    }
    text
}
