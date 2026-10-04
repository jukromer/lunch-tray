use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::model::{Meal, PriceGroup};

#[derive(Debug)]
pub struct MealRow {
    meal: Meal,
    price_group: PriceGroup,
    diet: Option<Diet>,
}

#[relm4::factory(pub)]
impl FactoryComponent for MealRow {
    type Init = (Meal, PriceGroup);
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &self.meal.name,
            set_subtitle: &subtitle(&self.meal),

            add_suffix = &gtk::Image {
                set_visible: self.diet.is_some(),
                set_icon_name: self.diet.map(Diet::icon_name),
                set_tooltip_text: self.diet.map(Diet::label),
                add_css_class: self.diet.map_or("dim-label", Diet::css_class),
            },

            add_suffix = &gtk::Label {
                set_label: &format_price(self.price_group.price(&self.meal.prices)),
                add_css_class: "numeric",
            },
        }
    }

    fn init_model(
        (meal, price_group): Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        let diet = diet(&meal.notes);
        Self {
            meal,
            price_group,
            diet,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Diet {
    Vegan,
    Vegetarian,
}

impl Diet {
    fn icon_name(self) -> &'static str {
        match self {
            Diet::Vegan => "leaf-symbolic",
            Diet::Vegetarian => "egg-symbolic",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Diet::Vegan => "Vegan",
            Diet::Vegetarian => "Vegetarian",
        }
    }

    fn css_class(self) -> &'static str {
        match self {
            Diet::Vegan => "success",
            Diet::Vegetarian => "warning",
        }
    }
}
fn diet(notes: &[String]) -> Option<Diet> {
    let notes = notes.join(" ").to_lowercase();
    if notes.contains("vegan") {
        Some(Diet::Vegan)
    } else if notes.contains("vegetar") || notes.contains("ohne fleisch") {
        Some(Diet::Vegetarian)
    } else {
        None
    }
}

fn format_price(price: Option<f64>) -> String {
    match price {
        Some(p) => format!("{:.2} €", p).replace(".", ","),
        None => String::new(),
    }
}

fn subtitle(meal: &Meal) -> String {
    let category = short_category(&meal.category);
    if meal.notes.is_empty() {
        category.to_string()
    } else {
        format!("{category} · {}", meal.notes.join(", "))
    }
}

fn short_category(category: &str) -> &str {
    match category.rsplit_once(" - ") {
        Some((name, suffix)) if looks_like_price(suffix) => name,
        _ => category,
    }
}

fn looks_like_price(text: &str) -> bool {
    text.replace(',', ".").parse::<f64>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Prices;

    fn meal(category: &str, notes: &[&str]) -> Meal {
        Meal {
            name: String::from("Kässpätzle"),
            category: String::from(category),
            prices: Prices {
                students: Some(3.3),
                employees: None,
                others: None,
            },
            notes: notes.iter().map(|note| note.to_string()).collect(),
        }
    }

    fn notes(notes: &[&str]) -> Vec<String> {
        notes.iter().map(|note| note.to_string()).collect()
    }

    #[test]
    fn detects_vegan() {
        assert_eq!(diet(&notes(&["Milch", "vegan"])), Some(Diet::Vegan));
    }

    #[test]
    fn detects_vegetarian() {
        assert_eq!(diet(&notes(&["vegetarisch"])), Some(Diet::Vegetarian));
    }

    #[test]
    fn ohne_fleisch_counts_as_vegetarian() {
        assert_eq!(diet(&notes(&["Ohne Fleisch"])), Some(Diet::Vegetarian));
    }

    #[test]
    fn meat_is_neither() {
        assert_eq!(diet(&notes(&["enthält Schweinefleisch"])), None);
    }

    #[test]
    fn subtitle_without_notes_is_category() {
        assert_eq!(
            subtitle(&meal("Tellergericht II - 3,90", &[])),
            "Tellergericht II"
        );
    }

    #[test]
    fn subtitle_lists_notes() {
        assert_eq!(
            subtitle(&meal("Beilage", &["vegan", "mit Knoblauch"])),
            "Beilage · vegan, mit Knoblauch"
        );
    }

    #[test]
    fn formats_price_with_comma() {
        assert_eq!(format_price(Some(3.1)), "3,10 €");
    }

    #[test]
    fn empty_without_price() {
        assert_eq!(format_price(None), "");
    }

    #[test]
    fn strips_price_from_category() {
        assert_eq!(short_category("Tellergericht I - 3,60"), "Tellergericht I");
    }

    #[test]
    fn keeps_category_without_price() {
        assert_eq!(short_category("Beilage"), "Beilage");
    }

    #[test]
    fn keeps_dash_that_is_not_a_price() {
        assert_eq!(short_category("Aktion - Pizza"), "Aktion - Pizza");
    }
}
