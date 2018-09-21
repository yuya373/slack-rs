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
use futures::Future;
use reqwest::async::Client;
use std::env;

fn main() {
    let token = env::var("TOKEN").expect("TOKEN='xoxp-foo...'");
    let client = Client::new();

    let request = api::rtm_connect_request(&token, &client).unwrap();
    let f = client
        .execute(request)
        .and_then(|mut response| response.json::<api::RtmConnectResponse>())
        .map_err(|err| println!("request error: {:?}", err))
        .map(|res| rtm::connect(&res));

    tokio::run(f);
}
