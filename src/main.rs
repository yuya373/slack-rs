extern crate reqwest;
extern crate serde;
extern crate ws;
#[macro_use]
extern crate serde_derive;

mod api;
mod rtm;
use std::env;

fn main() {
    let token = env::var("TOKEN").expect("TOKEN='xoxp-foo...'");
    let client = reqwest::Client::new();

    let response = api::rtm_connect(&token, &client).unwrap().json().unwrap();
    rtm::connect(&response);
}
