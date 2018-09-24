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
use futures::Stream;
use reqwest::async::Client;

#[derive(Debug)]
pub enum ActionType {
    Hello,
    Ping,
}
#[derive(Debug)]
pub struct Action {
    t: ActionType,
}
impl Action {
    pub fn ping() -> Action {
        Action {
            t: ActionType::Ping,
        }
    }
}
type Tx = mpsc::UnboundedSender<Action>;
// type Rx = mpsc::UnboundedReceiver<Action>;

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
                        resp.team.map(|team| workspace.set_team(team));
                        resp.me.map(|me| workspace.set_me(me));
                        let url = resp.url.clone().unwrap();
                        let mut conn = ws::Builder::new().build(rtm::Connection::new(tx)).unwrap();
                        let sender = conn.broadcaster();
                        let f = rx.for_each(move |action| {
                            match action.t {
                                ActionType::Ping => workspace.ping(&sender),
                                _ => {}
                            };
                            Ok(())
                        });
                        tokio::spawn(f);

                        conn.connect(url::Url::parse(&url).unwrap()).unwrap();
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
