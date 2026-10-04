use crate::model::Day;

const BASE_URL: &str = "https://openmensa.org/api/v2";

pub async fn fetch_meals(canteen_id: u32) -> Result<Vec<Day>, reqwest::Error> {
    let url = format!("{BASE_URL}/canteens/{canteen_id}/meals");
    reqwest::get(url).await?.error_for_status()?.json().await
}
