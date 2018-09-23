extern crate dirs;
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate tokio;
extern crate toml;
extern crate url;
extern crate ws;
#[macro_use]
extern crate serde_derive;

mod api;
mod config;
mod model;
mod rtm;
use api::RtmConnectResponse;
use futures::sync::mpsc;
use futures::Future;
use reqwest::async::Client;

#[derive(Debug)]
pub enum ActionType {
    Hello,
}
#[derive(Debug)]
pub struct Action {
    t: ActionType,
}
type Tx = mpsc::UnboundedSender<Action>;
type Rx = mpsc::UnboundedReceiver<Action>;

fn main() {
    let config = config::get_config().unwrap();
    let client = Client::new();
    // let workspace = config.workspaces[0];

    let f = futures::future::ok(config.workspaces).map(move |workspaces| {
        for mut workspace in workspaces {
            let (tx, rx) = mpsc::unbounded::<Action>();

            let f = api::rtm_connect_request(&workspace, &client)
                .send()
                .and_then(|mut res| res.json::<RtmConnectResponse>())
                .map(move |resp| {
                    if resp.ok {
                        workspace.merge(resp, rx);
                        let url = workspace.ws_url();
                        let mut conn = ws::Builder::new().build(rtm::Connection::new(tx)).unwrap();
                        conn.connect(url::Url::parse(&url).unwrap()).unwrap();
                        workspace.set_ws(conn.broadcaster());
                        tokio::spawn(workspace.process());
                        conn.run().unwrap();
                    } else {
                        panic!(resp.error.unwrap())
                    }
                }).map_err(|err| println!("Error: {}", err));
            tokio::spawn(f);
        }
    });
    tokio::run(f);
}
