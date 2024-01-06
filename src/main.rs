use std::error::Error;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

struct BetsApi {
    api_key: String,
    client: reqwest::Client
}

impl BetsApi {
    async fn new(api_key: &str) -> Result<Self,Box<dyn Error>> {
        let mut header = HeaderMap::new();
        header.insert("token", api_key.parse()?);
        let client = reqwest::Client::builder()
            .default_headers(header)
            .build()
            .expect("Unable to build the http client");
        // http request to see if the api key is valid
        let response = client.get(format!("https://api.b365api.com/v3/events/upcoming?sport_id=92&token={api_key}")).send().await?;
        if response.status() != StatusCode::OK {
            let error = format!("Error trying to initialize http connection to the server because it thrown error: {}, \
            you probably inserted an invalid api key",response.status());
            panic!("{}", error)
        }
        Ok(Self{
            api_key: String::from(api_key),
            client,
        })
    }

    fn get_upcoming_match(self, ) {

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let api = BetsApi::new("").await?;
    Ok(())
}
