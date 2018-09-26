use api::util::build_get;
use futures::Future;
use model::{Me, Team, Workspace};
use reqwest;
use reqwest::async::Client;

#[derive(Debug, Deserialize)]
pub struct ConnectResponse {
    pub ok: bool,
    pub url: Option<String>,
    pub team: Option<Team>,
    #[serde(rename = "self")]
    pub me: Option<Me>,
    pub error: Option<String>,
}

pub fn connect(
    workspace: &Workspace,
    client: Client,
) -> impl Future<Item = ConnectResponse, Error = reqwest::Error> {
    let url = "https://slack.com/api/rtm.connect";
    let query = [("mpim_aware", "1")];
    let builder = build_get(&client, url, &workspace.token);
    builder
        .query(&query)
        .send()
        .and_then(|mut resp| resp.json::<ConnectResponse>())
}
