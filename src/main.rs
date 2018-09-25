extern crate dirs;
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
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
use futures::sync::mpsc;
use futures::Future;
use futures::Stream;
use reqwest::async::Client;
use std::sync::Arc;

#[derive(Debug)]
pub enum ActionType {
    Hello,
    Ping,
}
#[derive(Debug)]
pub struct Action {
    Type: ActionType,
}
impl Action {
    pub fn hello() -> Action {
        Action {
            Type: ActionType::Hello,
        }
    }
    pub fn ping() -> Action {
        Action {
            Type: ActionType::Ping,
        }
    }
}
type Tx = mpsc::UnboundedSender<Action>;
// type Rx = mpsc::UnboundedReceiver<Action>;

fn main() {
    let config = config::get_config().unwrap();
    let client = Arc::new(Client::new());
    // let workspace = config.workspaces[0];

    let f = futures::future::ok(config.workspaces).map(move |workspaces| {
        for mut workspace in workspaces {
            let (tx, rx) = mpsc::unbounded::<Action>();
            let client = client.clone();

            let f = api::rtm::connect_request(&workspace, &client)
                .send()
                .and_then(|mut res| res.json::<api::rtm::ConnectResponse>())
                .map(|resp| {
                    if resp.ok {
                        resp.team.map(|team| workspace.set_team(team));
                        resp.me.map(|me| workspace.set_me(me));
                        let url = resp.url.clone().unwrap();
                        let mut conn = ws::Builder::new().build(rtm::Connection::new(tx)).unwrap();
                        let sender = conn.broadcaster();
                        let f = rx.for_each(move |action| {
                            match action.Type {
                                ActionType::Ping => workspace.ping(&sender),
                                ActionType::Hello => workspace.hello(&client),
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
