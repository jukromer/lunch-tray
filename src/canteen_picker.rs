use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::api;
use crate::model::Canteen;

const MAX_RESULTS: usize = 50;

pub struct CanteenPicker {
    canteens: Vec<Canteen>,
    query: String,
    results: Vec<Canteen>,
    rows: FactoryVecDeque<CanteenRow>,
    state: State,
}

#[derive(Debug)]
enum State {
    Loading,
    Ready,
    Failed(String),
}

#[derive(Debug)]
pub enum PickerMsg {
    Load,
    Search(String),
    Activated(usize),
}

#[derive(Debug)]
pub enum PickerOutput {
    Selected(Canteen),
}

#[derive(Debug)]
pub enum PickerCommand {
    Loaded(Result<Vec<Canteen>, reqwest::Error>),
}

#[relm4::component(pub)]
impl Component for CanteenPicker {
    type Init = ();
    type Input = PickerMsg;
    type Output = PickerOutput;
    type CommandOutput = PickerCommand;

    view! {
        adw::Dialog {
            set_title: "Choose Canteen",
            set_content_width: 420,
            set_content_height: 600,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                add_top_bar = &adw::Clamp {
                    set_margin_start: 12,
                    set_margin_end: 12,
                    set_margin_bottom: 6,

                    gtk::SearchEntry {
                        set_placeholder_text: Some("Search by name or city"),
                        connect_search_changed[sender] => move |entry| {
                            sender.input(PickerMsg::Search(entry.text().to_string()));
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
                        add_css_class: "compact",
                        set_icon_name: Some("network-error-symbolic"),
                        set_title: "Could Not Load Canteens",
                        #[watch]
                        set_description: model.error_message().as_deref(),

                        #[wrap(Some)]
                        set_child = &gtk::Button {
                            set_label: "Try Again",
                            set_halign: gtk::Align::Center,
                            add_css_class: "pill",
                            connect_clicked => PickerMsg::Load,
                        },
                    },

                    add_named[Some("empty")] = &adw::StatusPage {
                        add_css_class: "compact",
                        set_icon_name: Some("system-search-symbolic"),
                        set_title: "No Canteens Found",
                        set_description: Some("Try another name or city."),
                    },

                    add_named[Some("list")] = &gtk::ScrolledWindow {
                        set_hscrollbar_policy: gtk::PolicyType::Never,
                        set_vexpand: true,

                        adw::Clamp {
                            #[local_ref]
                            row_list -> gtk::ListBox {
                                set_selection_mode: gtk::SelectionMode::None,
                                set_valign: gtk::Align::Start,
                                set_margin_all: 12,
                                add_css_class: "boxed-list",
                                connect_row_activated[sender] => move |_, row| {
                                    sender.input(PickerMsg::Activated(row.index() as usize));
                                },
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
        let model = CanteenPicker {
            canteens: Vec::new(),
            query: String::new(),
            results: Vec::new(),
            rows: FactoryVecDeque::builder().launch_default().detach(),
            state: State::Loading,
        };
        let row_list = model.rows.widget();
        let widgets = view_output!();

        load_canteens(&sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            PickerMsg::Load => {
                if self.canteens.is_empty() && !matches!(self.state, State::Loading) {
                    self.state = State::Loading;
                    load_canteens(&sender);
                }
            }
            PickerMsg::Search(query) => {
                self.query = query;
                self.show_results();
            }
            PickerMsg::Activated(index) => {
                if let Some(canteen) = self.results.get(index) {
                    let _ = sender.output(PickerOutput::Selected(canteen.clone()));
                    root.close();
                }
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
            PickerCommand::Loaded(Ok(canteens)) => {
                self.canteens = canteens;
                self.state = State::Ready;
                self.show_results();
            }
            PickerCommand::Loaded(Err(error)) => {
                self.state = State::Failed(error.to_string());
            }
        }
    }
}

impl CanteenPicker {
    fn show_results(&mut self) {
        self.results = self
            .canteens
            .iter()
            .filter(|canteen| canteen.matches(&self.query))
            .take(MAX_RESULTS)
            .cloned()
            .collect();

        let mut rows = self.rows.guard();
        rows.clear();
        for canteen in &self.results {
            rows.push_back(canteen.clone());
        }
    }

    fn page(&self) -> &'static str {
        match self.state {
            State::Loading => "loading",
            State::Failed(_) => "failed",
            State::Ready if self.results.is_empty() => "empty",
            State::Ready => "list",
        }
    }

    fn error_message(&self) -> Option<String> {
        match &self.state {
            State::Failed(message) => Some(gtk::glib::markup_escape_text(message).to_string()),
            _ => None,
        }
    }
}

fn load_canteens(sender: &ComponentSender<CanteenPicker>) {
    sender.oneshot_command(async { PickerCommand::Loaded(api::fetch_canteens().await) });
}

#[derive(Debug)]
pub struct CanteenRow {
    canteen: Canteen,
}

#[relm4::factory(pub)]
impl FactoryComponent for CanteenRow {
    type Init = Canteen;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_activatable: true,
            set_title: &self.canteen.name,
            set_subtitle: &self.canteen.city,
        }
    }

    fn init_model(
        canteen: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { canteen }
    }
}
