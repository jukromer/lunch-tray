use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Day {
    pub date: NaiveDate,
    pub meals: Vec<Meal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meal {
    pub name: String,
    pub category: String,
    pub prices: Prices,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Prices {
    pub students: Option<f64>,
}
