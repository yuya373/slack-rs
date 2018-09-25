use api::util::build_get;
use model::{Me, Team, Workspace};
use reqwest::async::{Client, RequestBuilder};

#[derive(Debug, Deserialize)]
pub struct ConnectResponse {
    pub ok: bool,
    pub url: Option<String>,
    pub team: Option<Team>,
    #[serde(rename = "self")]
    pub me: Option<Me>,
    pub error: Option<String>,
}

pub fn connect_request(workspace: &Workspace, client: &Client) -> RequestBuilder {
    let url = "https://slack.com/api/rtm.connect";
    let query = [("mpim_aware", "1")];
    let builder = build_get(client, url, &workspace.token);
    builder.query(&query)
}
