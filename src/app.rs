use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::api;
use crate::meal_row::MealRow;
use crate::model::Day;

//ID of THA for early testing purpose
const CANTEEN_ID: u32 = 803;

pub struct App {
    state: State,
    days: Vec<Day>,
    selected: usize,
    meals: FactoryVecDeque<MealRow>,
}

#[derive(Debug)]
enum State {
    Loading,
    Ready,
    Failed(String),
}

#[derive(Debug)]
pub enum AppMsg {
    Reload,
    PreviousDay,
    NextDay,
}

#[derive(Debug)]
pub enum CommandMsg {
    Loaded(Result<Vec<Day>, reqwest::Error>),
}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = AppMsg;
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
                        set_subtitle: &model.subtitle(),
                    },

                    pack_start = &gtk::Button {
                        set_icon_name: "go-previous-symbolic",
                        set_tooltip_text: Some("Previous Day"),
                        #[watch]
                        set_sensitive: model.selected > 0,
                        connect_clicked => AppMsg::PreviousDay,
                    },

                    pack_end = &gtk::Button {
                        set_icon_name: "go-next-symbolic",
                        set_tooltip_text: Some("Next Day"),
                        #[watch]
                        set_sensitive: model.selected + 1 < model.days.len(),
                        connect_clicked => AppMsg::NextDay,
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::Stack {
                    add_named[Some("loading")] = &adw::Spinner {
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                        set_width_request: 32,
                        set_height_request: 32,
                    },

                    add_named[Some("failed")] = &adw::StatusPage {
                        set_icon_name: Some("network-error-symbolic"),
                        set_title: "Could Not Load the Menu",
                        #[watch]
                        set_description: model.error_message(),

                        #[wrap(Some)]
                        set_child = &gtk::Button {
                            set_label: "Try Again",
                            set_halign: gtk::Align::Center,
                            add_css_class: "pill",
                            connect_clicked => AppMsg::Reload,
                        },
                    },

                    add_named[Some("empty")] = &adw::StatusPage {
                        set_icon_name: Some("emoji-food-symbolic"),
                        set_title: "No Meals",
                        set_description: Some("There is no menu for this day."),
                    },

                    add_named[Some("meals")] = &gtk::ScrolledWindow {
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

                    #[watch]
                    set_visible_child_name: model.page(),
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
            state: State::Loading,
            days: Vec::new(),
            selected: 0,
            meals: FactoryVecDeque::builder().launch_default().detach(),
        };
        let meal_list = model.meals.widget();
        let widgets = view_output!();

        load_meals(&sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            AppMsg::Reload => {
                self.state = State::Loading;
                load_meals(&sender);
            }
            AppMsg::PreviousDay => {
                self.selected = previous_index(self.selected);
                self.show_selected_day();
            }
            AppMsg::NextDay => {
                self.selected = next_index(self.selected, self.days.len());
                self.show_selected_day();
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            CommandMsg::Loaded(Ok(days)) => {
                self.days = days;
                self.selected = 0;
                self.state = State::Ready;
                self.show_selected_day();
            }
            CommandMsg::Loaded(Err(error)) => {
                self.state = State::Failed(error.to_string());
            }
        }
    }
}

impl App {
    fn show_selected_day(&mut self) {
        let mut meals = self.meals.guard();
        meals.clear();
        if let Some(day) = self.days.get(self.selected) {
            for meal in &day.meals {
                meals.push_back(meal.clone());
            }
        }
    }

    fn page(&self) -> &'static str {
        match self.state {
            State::Loading => "loading",
            State::Failed(_) => "failed",
            State::Ready if self.meals.is_empty() => "empty",
            State::Ready => "meals",
        }
    }

    fn error_message(&self) -> Option<&str> {
        match &self.state {
            State::Failed(message) => Some(message),
            _ => None,
        }
    }

    fn subtitle(&self) -> String {
        match self.days.get(self.selected) {
            Some(day) => day.date.format("%A, %B %-d").to_string(),
            None => String::new(),
        }
    }
}

fn load_meals(sender: &ComponentSender<App>) {
    sender.oneshot_command(async { CommandMsg::Loaded(api::fetch_meals(CANTEEN_ID).await) });
}

fn previous_index(selected: usize) -> usize {
    if selected == 0 { 0 } else { selected - 1 }
}

fn next_index(selected: usize, day_count: usize) -> usize {
    if selected + 1 < day_count {
        selected + 1
    } else if day_count == 0 {
        0
    } else {
        day_count - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_goes_back_one_day() {
        assert_eq!(previous_index(2), 1);
    }

    #[test]
    fn previous_stays_on_first_day() {
        assert_eq!(previous_index(0), 0);
    }

    #[test]
    fn next_goes_forward_one_day() {
        assert_eq!(next_index(0, 5), 1);
    }

    #[test]
    fn next_stays_on_last_day() {
        assert_eq!(next_index(4, 5), 4);
    }

    #[test]
    fn next_without_days_stays_at_zero() {
        assert_eq!(next_index(0, 0), 0);
    }
}
