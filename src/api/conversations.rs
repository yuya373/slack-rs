use api::util::{build_get, ResponseMetadata};
use futures::future::{loop_fn, Loop};
use futures::Future;
use reqwest;
use reqwest::async::Client;
use serde;

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
    fn empty(cursor: String) -> ListResponse<T> {
        ListResponse {
            ok: true,
            channels: Vec::new(),
            response_metadata: ResponseMetadata {
                next_cursor: cursor,
            },
        }
    }
}

pub fn get_list<T>(
    client: Client,
    token: String,
    types: ListType,
    cursor: String,
) -> impl Future<Item = ListResponse<T>, Error = reqwest::Error>
where
    for<'de> T: serde::Deserialize<'de>,
{
    let types = match types {
        ListType::All => "public_channel,private_channel,im,mpim",
        ListType::Private => "private_channel",
        ListType::Public => "public_channel",
        ListType::Im => "im",
        ListType::Mpim => "mpim",
    };
    println!("get_list");

    loop_fn(
        (client, token, ListResponse::empty(cursor)),
        move |(client, token, mut res)| {
            let url = "https://slack.com/api/conversations.list";
            let next_cursor = res.response_metadata.next_cursor.clone();
            let query = [("types", types), ("limit", "10"), ("cursor", &next_cursor)];
            let req = build_get(&client.clone(), url, &token).query(&query);

            println!("get_list:req: {:?}", req);
            req.send()
                .and_then(|mut res| res.json::<ListResponse<T>>())
                .and_then(|mut new_res| {
                    println!("get_list:meta: {:?}", new_res.response_metadata);
                    res.channels.append(&mut new_res.channels);
                    res.response_metadata = new_res.response_metadata;
                    res.ok = new_res.ok;

                    if res.ok && res.response_metadata.next_cursor.len() > 0 {
                        Ok(Loop::Continue((client, token, res)))
                    } else {
                        Ok(Loop::Break(res))
                    }
                })
        },
    )
}
