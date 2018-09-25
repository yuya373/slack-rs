use api::util::{build_get, ResponseMetadata};
use reqwest::async::{Client, RequestBuilder};

pub enum ListType {
    All,
    Private,
    Public,
    Im,
    Mpim,
}

#[derive(Debug, Deserialize)]
pub struct ListResponse<T> {
    pub ok: bool,
    pub channels: Vec<T>,
    pub response_metadata: ResponseMetadata,
}

impl<T> ListResponse<T> {
    pub fn empty() -> ListResponse<T> {
        ListResponse {
            ok: true,
            channels: Vec::new(),
            response_metadata: ResponseMetadata {
                next_cursor: "".to_string(),
            },
        }
    }
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
        ("limit", "10"),
        (
            "cursor",
            match cursor {
                Some(s) => s,
                None => "",
            },
        ),
    ];
    println!("query: {:?}", query);
    let builder = build_get(client, url, token);
    builder.query(&query)
}
