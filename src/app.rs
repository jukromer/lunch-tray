use relm4::adw::prelude::*;
use relm4::prelude::*;

pub struct App {}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Lunch Tray"),
            set_default_size: (420, 640),

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &adw::StatusPage {
                    set_title: "Lunch Tray",
                    set_description: Some("The menu will show up here."),
                },
            },
        }
    }
    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
