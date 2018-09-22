extern crate dirs;
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate tokio;
extern crate toml;
extern crate ws;
#[macro_use]
extern crate serde_derive;

mod api;
mod config;
mod model;
mod rtm;
use api::RtmConnectResponse;
use futures::Future;
use reqwest::async::{Client, Response};

fn main() {
    let config = config::get_config().unwrap();
    let client = Client::new();
    // let workspace = config.workspaces[0];
    let f = futures::future::ok(config.workspaces).map(move |workspaces| {
        for workspace in workspaces {
            let json = |mut response: Response| response.json::<RtmConnectResponse>();
            let f = api::rtm_connect_request(&workspace, &client)
                .send()
                .and_then(json)
                .map(|resp| {
                    if resp.ok {
                        rtm::connect(&resp)
                    } else {
                        panic!(resp.error.unwrap())
                    }
                }).map_err(|err| println!("Error: {}", err));
            tokio::spawn(f);
        }
    });
    tokio::run(f);
}
