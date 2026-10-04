use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::model::Meal;

#[derive(Debug)]
pub struct MealRow {
    meal: Meal,
}

#[relm4::factory(pub)]
impl FactoryComponent for MealRow {
    type Init = Meal;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &self.meal.name,
            set_subtitle: &self.meal.category,

            add_suffix = &gtk::Label {
                set_label: &format_price(self.meal.prices.students),
                add_css_class: "numeric",
            },
        }
    }

    fn init_model(meal: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { meal }
    }
}

fn format_price(price: Option<f64>) -> String {
    match price {
        Some(p) => format!("{:.2} €", p).replace(".", ","),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_price_with_comma() {
        assert_eq!(format_price(Some(3.1)), "3,10 €");
    }

    #[test]
    fn empty_without_price() {
        assert_eq!(format_price(None), "");
    }
}
