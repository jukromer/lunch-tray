use std::fs;
use std::path::PathBuf;

use relm4::gtk::glib;
use serde::{Deserialize, Serialize};

use crate::model::{Canteen, PriceGroup};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub canteen: Canteen,
    pub price_group: PriceGroup,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            canteen: Canteen {
                id: 803,
                name: String::from("Mensa TH Augsburg"),
                city: String::from("Augsburg"),
            },
            price_group: PriceGroup::Students,
        }
    }
}

fn config_path() -> PathBuf {
    glib::user_config_dir().join("lunch-tray").join("config.json")
}

pub fn load() -> Config {
    match fs::read_to_string(config_path()) {
        Ok(text) => parse(&text),
        Err(_) => Config::default(),
    }
}

pub fn save(config: &Config) -> std::io::Result<()> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)
}

fn parse(text: &str) -> Config {
    serde_json::from_str(text).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_saved_config() {
        let json = r#"{
            "canteen": { "id": 802, "name": "Mensa Uni Augsburg", "city": "Augsburg" },
            "price_group": "Employees"
        }"#;
        let config = parse(json);
        assert_eq!(config.canteen.id, 802);
        assert_eq!(config.price_group, PriceGroup::Employees);
    }

    #[test]
    fn falls_back_on_broken_json() {
        assert_eq!(parse("{ kaputt").canteen.id, 803);
    }

    #[test]
    fn falls_back_on_empty_file() {
        assert_eq!(parse("").price_group, PriceGroup::Students);
    }

    #[test]
    fn reads_what_save_writes() {
        let json = serde_json::to_string_pretty(&Config::default()).unwrap();
        assert_eq!(parse(&json).canteen.name, "Mensa TH Augsburg");
    }
}
