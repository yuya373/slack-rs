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

use api::conversations::{ListResponse, ListType};
use futures::future::{loop_fn, Loop};
use futures::sync::mpsc;
use futures::Future;
use futures::Stream;
use model::{Channel, Group, Im, Me, Mpim, Team};
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
    // let client = client.clone();
    // let workspace = config.workspaces[0];

    let f = futures::future::ok(config.workspaces).map(move |workspaces| {
        for mut workspace in workspaces {
            let (tx, rx) = mpsc::unbounded::<Action>();

            let c = client.clone();
            let f = api::rtm::connect_request(&workspace, &c)
                .send()
                .and_then(|mut res| res.json::<api::rtm::ConnectResponse>())
                .map(move |resp| {
                    if resp.ok {
                        resp.team.map(|team| workspace.set_team(team));
                        resp.me.map(|me| workspace.set_me(me));
                        let url = resp.url.clone().unwrap();
                        let mut conn = ws::Builder::new().build(rtm::Connection::new(tx)).unwrap();
                        let sender = conn.broadcaster();
                        let workspace = Arc::new(std::sync::Mutex::new(workspace));
                        let f = rx.for_each(move |action| {
                            let workspace = workspace.clone();
                            let c = c.clone();

                            match action.Type {
                                ActionType::Ping => workspace.lock().unwrap().ping(&sender),
                                ActionType::Hello => {
                                    let token = workspace.lock().unwrap().token.clone();

                                    let public_channels =
                                        loop_fn(ListResponse::empty(), move |mut res| {
                                            let next_cursor =
                                                res.response_metadata.next_cursor.clone();

                                            api::conversations::list_request(
                                                &token,
                                                &c,
                                                ListType::Public,
                                                Some(&next_cursor),
                                            ).send()
                                            .and_then(|mut res| res.json::<ListResponse<Channel>>())
                                            .and_then(|mut new_res| {
                                                res.channels.append(&mut new_res.channels);
                                                res.response_metadata = new_res.response_metadata;
                                                res.ok = new_res.ok;

                                                if res.ok
                                                    && res.response_metadata.next_cursor.len() > 0
                                                {
                                                    Ok(Loop::Continue(res))
                                                } else {
                                                    Ok(Loop::Break(res))
                                                }
                                            }).map_err(
                                                |err| {
                                                    println!("Error in public_channels: {:?}", err);
                                                },
                                            )
                                        });

                                    tokio::spawn(public_channels.map(
                                        move |res: ListResponse<Channel>| {
                                            let mut workspace = workspace.lock().unwrap();
                                            workspace.set_channels(res.channels);
                                            println!("finished: {:?}", workspace.channels.len());
                                        },
                                    ));
                                }
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
