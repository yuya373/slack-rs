use api::util::{build_get, ResponseMetadata};
use reqwest::async::{Client, RequestBuilder};
use serde_json::Value;

pub enum ListType {
    All,
    Private,
    Public,
    Im,
    Mpim,
}

#[derive(Debug, Deserialize)]
struct ListResponse {
    ok: bool,
    channels: Vec<Value>,
    response_metadata: ResponseMetadata,
}

pub fn list_request(
    token: &str,
    client: &Client,
    types: ListType,
    cursor: Option<&str>,
) -> RequestBuilder {
    let url = "https://slack.com/api/conversations.list";
    let types = match types {
        ListType::All => "public_channel,private_channel,im,mpim",
        ListType::Private => "private_channel",
        ListType::Public => "public_channel",
        ListType::Im => "im",
        ListType::Mpim => "mpim",
    };
    let query = [
        ("types", types),
        ("limit", "100"),
        (
            "cursor",
            match cursor {
                Some(s) => s,
                None => "",
            },
        ),
    ];
    let builder = build_get(client, url, token);
    builder.query(&query)
}
