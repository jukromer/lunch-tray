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
}

#[derive(Debug)]
pub enum PickerMsg {
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
                set_content = &gtk::ScrolledWindow {
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
        };
        let row_list = model.rows.widget();
        let widgets = view_output!();

        sender.oneshot_command(async { PickerCommand::Loaded(api::fetch_canteens().await) });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
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
                self.show_results();
            }
            PickerCommand::Loaded(Err(error)) => {
                eprintln!("Could not load canteens: {error}");
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
