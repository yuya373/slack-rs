extern crate reqwest;

use model::{Me, Team, Workspace};
use reqwest::async::{Client, Request, RequestBuilder, Response};
use reqwest::header::HeaderMap;
use reqwest::Result;

#[derive(Debug, Deserialize)]
pub struct RtmConnectResponse {
    pub ok: bool,
    pub url: Option<String>,
    team: Option<Team>,
    #[serde(rename = "self")]
    me: Option<Me>,
    pub error: Option<String>,
}

fn build_get(client: &Client, url: &str, token: &str) -> RequestBuilder {
    let mut h = HeaderMap::new();
    h.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    client.get(url).headers(h)
}

pub fn rtm_connect_request(workspace: &Workspace, client: &Client) -> RequestBuilder {
    let url = "https://slack.com/api/rtm.connect";
    let query = [("mpim_aware", "1")];
    let builder = build_get(client, url, &workspace.token);
    builder.query(&query)
}

pub enum ConversationsListTypes {
    All,
    Private,
    Public,
    Im,
    Mpim,
}

pub fn conversations_list_request(
    token: &str,
    client: &Client,
    types: ConversationsListTypes,
    cursor: Option<&str>,
) -> Result<Request> {
    let url = "https://slack.com/api/conversations.list";
    let types = match types {
        ConversationsListTypes::All => "public_channel,private_channel,im,mpim",
        ConversationsListTypes::Private => "private_channel",
        ConversationsListTypes::Public => "public_channel",
        ConversationsListTypes::Im => "im",
        ConversationsListTypes::Mpim => "mpim",
    };
    let query = [(
        "types",
        types,
        "limit",
        100,
        "cursor",
        match cursor {
            Some(s) => s,
            None => "",
        },
    )];
    let builder = build_get(client, url, token);
    builder.query(&query).build()
}
