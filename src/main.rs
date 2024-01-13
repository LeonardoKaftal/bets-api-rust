use std::error::Error;
use std::fmt::Debug;
use chrono::{NaiveDate};
use reqwest::header::{HeaderMap};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

struct BetsApi {
    api_key: String,
    client: reqwest::Client
}

#[derive(Serialize, Deserialize)]
struct UpcomingMatchesResults {
    success: u8,
    pager: Pager,
    results: Vec<UpcomingMatch>
}

#[derive(Serialize, Deserialize)]
struct Pager {
    page: u8,
    per_page: u8,
    total: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct UpcomingMatch {
    id: String,
    sport_id: String,
    time: String,
    time_status: Option<String>,
    league: Option<League>,
    home: Option<HomeAway>,
    away: Option<HomeAway>,
    ss: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct League {
    id: String,
    name: String,
    cc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HomeAway {
    id: String,
    name: String,
    image_id: Option<i32>,
    cc: Option<String>,
}

impl BetsApi {
    async fn new(api_key: &str) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::builder()
            .default_headers(HeaderMap::new())
            .build()?;

        // http request to see if the api key is valid
        let response = client
            .get(format!("https://api.b365api.com/v3/events/upcoming?sport_id=92&token={}", api_key))
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Err(format!(
                "Error trying to initialize http connection to the server because it thrown error: {}",
                response.status()
            ).into());
        }

        Ok(Self {
            api_key: api_key.to_string(),
            client,
        })
    }

    async fn get_upcoming_match(
        &self,
        sport_id: &str,
        league_id: Option<String>,
        team_id: Option<String>,
        cc: Option<String>,
        day: Option<NaiveDate>,
        skip_esports: Option<String>, ) -> Result<Vec<UpcomingMatch>, Box<dyn Error>> {
            let league_id = league_id.unwrap_or_else(|| String::new());
            let team_id = team_id.unwrap_or_else(|| String::new());
            let cc = cc.unwrap_or_else(|| String::new());
            let skip_esports = skip_esports.unwrap_or_else(|| String::new());

            let mut matches = Vec::new();
            let mut page = 1;


            loop {
                let mut url = format!(
                    "https://api.b365api.com/v3/events/upcoming?sport_id={}&token={}&league_id={}&team_id={}&cc={}&skip_esports={}&page={}",
                    sport_id, self.api_key, league_id, team_id, cc, skip_esports,page
                );

                if let Some(day) = day {
                    let date = day.format("20%y%m%d").to_string();
                    url.push_str(&format!("&day={}", date));
                }

                let response = self.client.get(&url).send().await?;
                let upcoming_matches: UpcomingMatchesResults = response.json().await?;
                if !upcoming_matches.results.is_empty() {
                    matches.extend(upcoming_matches.results);
                }
                else {
                    break;
                }
                page += 1;
            }

            Ok(matches)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api = BetsApi::new("").await?;
    Ok(())
}
