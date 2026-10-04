use std::sync::LazyLock;
use std::time::Duration;

use crate::model::{Canteen, Day};

const BASE_URL: &str = "https://openmensa.org/api/v2";

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent(concat!(
            "LunchTray/",
            env!("CARGO_PKG_VERSION"),
            " (https://github.com/jukromer/lunch-tray)"
        ))
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Failed to create the HTTP client")
});

pub async fn fetch_meals(canteen_id: u32) -> Result<Vec<Day>, reqwest::Error> {
    let url = format!("{BASE_URL}/canteens/{canteen_id}/meals");
    CLIENT
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

pub async fn fetch_canteens() -> Result<Vec<Canteen>, reqwest::Error> {
    let mut canteens = Vec::new();
    let mut page = 1;
    loop {
        let url = format!("{BASE_URL}/canteens?limit=500&page={page}");
        let response = CLIENT.get(url).send().await?.error_for_status()?;
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
