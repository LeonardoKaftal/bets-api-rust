use std::collections::HashMap;
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

#[derive(Serialize, Deserialize, Debug)]
struct MatchResult {
    success: u8,
    pager: Pager,
    results: Vec<ApiMatches>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pager {
    page: u8,
    per_page: u8,
    total: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiMatches {
    id: Option<String>,
    sport_id: Option<String>,
    time: Option<String>,
    time_status: Option<String>,
    league: Option<League>,
    home: Option<HomeAway>,
    o_home: Option<HomeAway>,
    away: Option<HomeAway>,
    o_away: Option<HomeAway>,
    ss: Option<String>,
    scores: Option<HashMap<String, Score>>,
    bet365id: Option<String>,
    // attribute for specific sport parameter like soccer player stats which are not available in other sports
    #[serde(flatten)]
    extra: Option<serde_json::Value>
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



#[derive(Serialize, Deserialize, Debug)]
struct Score {
    home: Option<String>,
    away: Option<String>,
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
        skip_esports: Option<String>) -> Result<Vec<ApiMatches>, Box<dyn Error>> {
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
                let upcoming_matches: MatchResult = response.json().await?;
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

    async fn get_played_match(&self,
                              sport_id: &str,
                              league_id: Option<String>,
                              team_id: Option<String>,
                              cc: Option<String>,
                              day: Option<NaiveDate>,
                              skip_esports: Option<String>) -> Result<Vec<ApiMatches>, Box<dyn Error>> {

        let league_id = league_id.unwrap_or_else(|| String::new());
        let team_id = team_id.unwrap_or_else(|| String::new());
        let cc = cc.unwrap_or_else(|| String::new());
        let skip_esports = skip_esports.unwrap_or_else(|| String::new());

        let mut matches = Vec::new();
        let mut page = 1;


        loop {
            let mut url = format!(
                "https://api.b365api.com/v3/events/ended?sport_id={}&token={}&league_id={}&team_id={}&cc={}&skip_esports={}&page={}",
                sport_id, self.api_key, league_id, team_id, cc, skip_esports,page
            );

            if let Some(day) = day {
                let date = day.format("20%y%m%d").to_string();
                url.push_str(&format!("&day={}", date));
            }

            let response = self.client.get(&url).send().await?;
            let played_matches: MatchResult = response.json().await?;
            if !played_matches.results.is_empty() {
                matches.extend(played_matches.results);
            }
            else {
                break;
            }
            page += 1;
        }

        Ok(matches)
    }

    async fn get_in_play_match(&self,
                               sport_id: &str,
                               league_id: Option<String>) -> Result<ApiMatches,Box<dyn Error>> {
        let league_id = league_id.unwrap_or_else(|| String::new());
        let url = format!("https://api.b365api.com/v3/events/inplay?sport_id={}&token={}&league_id={}",sport_id,self.api_key,league_id);
        let response = self.client.get(url).send().await?;
        let matches: ApiMatches = response.json().await?;
        Ok(matches)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api = BetsApi::new("").await?;
    Ok(())
}
