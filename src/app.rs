use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::api;
use crate::canteen_picker::{CanteenPicker, PickerOutput};
use crate::config::{self, Config};
use crate::meal_row::MealRow;
use crate::model::{Canteen, Day, PriceGroup};

pub struct App {
    canteen: Canteen,
    picker: Controller<CanteenPicker>,
    state: State,
    days: Vec<Day>,
    selected: usize,
    price_group: PriceGroup,
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
    SetPriceGroup(PriceGroup),
    OpenPicker,
    SelectCanteen(Canteen),
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
                        #[watch]
                        set_title: &model.canteen.name,
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

                    pack_end = &gtk::Button {
                        set_icon_name: "find-location-symbolic",
                        set_tooltip_text: Some("Choose Canteen"),
                        connect_clicked => AppMsg::OpenPicker,
                    },
                },
                add_top_bar = &adw::Clamp {
                    set_maximum_size: 400,
                    set_margin_start: 12,
                    set_margin_end: 12,
                    set_margin_bottom: 6,

                    adw::ToggleGroup {
                        set_homogeneous: true,

                        add = adw::Toggle {
                            set_label: Some("Students"),
                            set_name: Some(PriceGroup::Students.name()),
                        },
                        add = adw::Toggle {
                            set_label: Some("Employees"),
                            set_name: Some(PriceGroup::Employees.name()),
                        },
                        add = adw::Toggle {
                            set_label: Some("Guests"),
                            set_name: Some(PriceGroup::Guests.name()),
                        },

                        set_active_name: Some(model.price_group.name()),
                        connect_active_name_notify[sender] => move |group| {
                            let name = group.active_name().unwrap_or_default();
                            if let Some(price_group) = PriceGroup::from_name(&name) {
                                sender.input(AppMsg::SetPriceGroup(price_group));
                            }
                        },
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
        let config = config::load();
        let picker = CanteenPicker::builder()
            .launch(())
            .forward(sender.input_sender(), |output| match output {
                PickerOutput::Selected(canteen) => AppMsg::SelectCanteen(canteen),
            });
        let model = App {
            canteen: config.canteen,
            picker,
            state: State::Loading,
            days: Vec::new(),
            selected: 0,
            price_group: config.price_group,
            meals: FactoryVecDeque::builder().launch_default().detach(),
        };
        let meal_list = model.meals.widget();
        let widgets = view_output!();

        load_meals(&sender, model.canteen.id);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            AppMsg::Reload => {
                self.state = State::Loading;
                load_meals(&sender, self.canteen.id);
            }
            AppMsg::PreviousDay => {
                self.selected = previous_index(self.selected);
                self.show_selected_day();
            }
            AppMsg::NextDay => {
                self.selected = next_index(self.selected, self.days.len());
                self.show_selected_day();
            }
            AppMsg::SetPriceGroup(price_group) => {
                self.price_group = price_group;
                self.show_selected_day();
                self.save_config();
            }
            AppMsg::OpenPicker => {
                self.picker.widget().present(Some(root));
            }
            AppMsg::SelectCanteen(canteen) => {
                self.canteen = canteen;
                self.state = State::Loading;
                load_meals(&sender, self.canteen.id);
                self.save_config();
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
    fn save_config(&self) {
        let config = Config {
            canteen: self.canteen.clone(),
            price_group: self.price_group,
        };
        if let Err(error) = config::save(&config) {
            eprintln!("Could not save settings: {error}");
        }
    }

    fn show_selected_day(&mut self) {
        let mut meals = self.meals.guard();
        meals.clear();
        if let Some(day) = self.days.get(self.selected) {
            for meal in &day.meals {
                meals.push_back((meal.clone(), self.price_group));
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

fn load_meals(sender: &ComponentSender<App>, canteen_id: u32) {
    sender.oneshot_command(async move { CommandMsg::Loaded(api::fetch_meals(canteen_id).await) });
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
