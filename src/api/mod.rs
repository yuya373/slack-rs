extern crate reqwest;
use reqwest::{Client, Response, Result};
use model::{Me, Team};

#[derive(Debug, Deserialize)]
pub struct RtmConnectResponse {
    ok: bool,
    pub url: String,
    team: Team,
    #[serde(rename = "self")]
    me: Me,
}
#[derive(Debug, Deserialize)]
struct Team {
    id: String,
    name: String,
    domain: String,
}
#[derive(Debug, Deserialize)]
struct Me {
    id: String,
    name: String,
}

pub fn rtm_connect(token: &str, client: &Client) -> Result<Response> {
    let url = "https://slack.com/api/rtm.connect";
    let builder = client.get(url);
    let query = [("mpim_aware", "1")];

    builder.bearer_auth(token).query(&query).send()
}
