use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::api;
use crate::meal_row::MealRow;
use crate::model::Day;

//ID of THA for early testing prupose
const CANTEEN_ID: u32 = 803;

pub struct App {
    subtitle: String,
    meals: FactoryVecDeque<MealRow>,
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
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Mensa TH Augsburg",
                        #[watch]
                        set_subtitle: &model.subtitle,
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,

                    adw::Clamp {
                        #[local_ref]
                        meal_list -> gtk::ListBox {
                            set_selection_mode: gtk::SelectionMode::None,
                            set_valign: gtk::Align::Start,
                            set_margin_all: 12,
                            add_css_class: "boxed-list",
                        },
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
            subtitle: String::from("Loading..."),
            meals: FactoryVecDeque::builder().launch_default().detach(),
        };
        let meal_list = model.meals.widget();
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
                if let Some(day) = days.first() {
                    self.subtitle = day.date.format("%A, %B %-d").to_string();
                    let mut meals = self.meals.guard();
                    for meal in &day.meals {
                        meals.push_back(meal.clone());
                    }
                } else {
                    self.subtitle = String::from("No menu available");
                }
            }
            CommandMsg::Loaded(Err(error)) => {
                self.subtitle = format!("Error: {error}");
            }
        }
    }
}
