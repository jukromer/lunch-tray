use crate::model::{Canteen, Day};

const BASE_URL: &str = "https://openmensa.org/api/v2";

pub async fn fetch_meals(canteen_id: u32) -> Result<Vec<Day>, reqwest::Error> {
    let url = format!("{BASE_URL}/canteens/{canteen_id}/meals");
    reqwest::get(url).await?.error_for_status()?.json().await
}

pub async fn fetch_canteens() -> Result<Vec<Canteen>, reqwest::Error> {
    let mut canteens = Vec::new();
    let mut page = 1;
    loop {
        let url = format!("{BASE_URL}/canteens?limit=500&page={page}");
        let response = reqwest::get(url).await?.error_for_status()?;
        let total_pages: u32 = response
            .headers()
            .get("x-total-pages")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        let mut batch: Vec<Canteen> = response.json().await?;
        canteens.append(&mut batch);
        if page >= total_pages {
            break;
        }
        page += 1;
    }
    Ok(canteens)
}
