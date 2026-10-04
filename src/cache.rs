use std::fs;
use std::path::PathBuf;

use chrono::NaiveDate;
use relm4::gtk::glib;

use crate::model::Day;

fn cache_path(canteen_id: u32) -> PathBuf {
    glib::user_cache_dir()
        .join("lunch-tray")
        .join(format!("meals-{canteen_id}.json"))
}

pub fn save(canteen_id: u32, days: &[Day]) -> std::io::Result<()> {
    let path = cache_path(canteen_id);
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string(days)?;
    fs::write(path, json)
}

pub fn load(canteen_id: u32) -> Option<Vec<Day>> {
    let text = fs::read_to_string(cache_path(canteen_id)).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn upcoming(days: Vec<Day>, today: NaiveDate) -> Vec<Day> {
    days.into_iter().filter(|day| day.date >= today).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(date: &str) -> Day {
        Day {
            date: date.parse().unwrap(),
            meals: Vec::new(),
        }
    }

    fn week() -> Vec<Day> {
        vec![day("2026-10-05"), day("2026-10-06"), day("2026-10-07")]
    }

    #[test]
    fn drops_past_days() {
        let days = upcoming(week(), "2026-10-06".parse().unwrap());
        assert_eq!(days.len(), 2);
        assert_eq!(days[0].date.to_string(), "2026-10-06");
    }

    #[test]
    fn keeps_all_days_before_the_week() {
        assert_eq!(upcoming(week(), "2026-10-04".parse().unwrap()).len(), 3);
    }

    #[test]
    fn empty_when_all_days_are_past() {
        assert!(upcoming(week(), "2026-10-10".parse().unwrap()).is_empty());
    }
}
