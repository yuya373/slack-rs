extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate tokio;
extern crate ws;
#[macro_use]
extern crate serde_derive;

mod api;
mod model;
mod rtm;
use api::RtmConnectResponse;
use futures::Future;
use model::Workspace;
use reqwest::async::{Client, Response};
use std::env;

fn main() {
    let token = env::var("TOKEN").expect("TOKEN='xoxp-foo...'");
    let workspace = Workspace::new(&token);
    let client = Client::new();

    let json = |mut response: Response| response.json::<RtmConnectResponse>();
    let f = api::rtm_connect_request(&workspace, &client)
        .send()
        .and_then(json)
        .map(|resp| {
            if resp.ok {
                println!("rtm::connect");
                rtm::connect(&resp);
            } else {
                panic!("{:?} ", resp.error.unwrap());
            }
        }).map_err(|err| println!("Error: {:?}", err));

    tokio::run(f);
}
