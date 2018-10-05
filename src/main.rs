#![recursion_limit = "128"]
extern crate dirs;
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate tokio_timer;
extern crate tokio_tls;
extern crate tokio_tungstenite;
extern crate toml;
extern crate tungstenite;
extern crate url;
#[macro_use]
extern crate serde_derive;

mod api;
mod config;
mod model;
mod rtm;

use futures::sync::mpsc;
use futures::Future;
use futures::Sink;
use futures::Stream;
use model::{Channel, Group, Im, Me, Mpim, Team, Workspace};
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
    // let client = Arc::new(Client::new());
    // let workspace = config.workspaces[0];
    let workspaces = config.workspaces;
    let mut fs = Vec::new();

    for mut workspace in workspaces {
        let client = Client::new();
        fs.push(
            api::rtm::connect(&workspace, client.clone())
                .map_err(|err| println!("Failed to api::rtm::connect: {:?}", err))
                .and_then(|resp| {
                    resp.team.map(|team| workspace.set_team(team));
                    resp.me.map(|me| workspace.set_me(me));
                    let (tx, rx) = mpsc::unbounded::<Action>();
                    let url = resp.url.clone().unwrap();

                    let url = url::Url::parse(&url).unwrap();
                    tokio_tungstenite::connect_async(url)
                        .map_err(|err| println!("failed to connect: {:?}", err))
                        .and_then(move |(ws_stream, _)| {
                            let (sender, stream) = ws_stream.split();
                            let workspace = Arc::new(std::sync::Mutex::new(workspace));
                            let sender = Arc::new(std::sync::Mutex::new(sender));

                            let f = rx.for_each(move |action| {
                                let workspace = workspace.clone();
                                // let sender = sender.clone();

                                match action.Type {
                                    ActionType::Ping => {
                                        let mut workspace = workspace.lock().unwrap();
                                        let mut sender = sender.lock().unwrap();
                                        Workspace::handle_ping(&mut sender, &mut workspace)
                                            .unwrap();
                                    }
                                    ActionType::Hello => {
                                        let token = workspace.lock().unwrap().token.clone();
                                        let f = Workspace::handle_hello(token, client.clone())
                                            .map(move |(public, private, im, mpim)| {
                                                let mut workspace = workspace.lock().unwrap();
                                                workspace.set_channels(public.channels);
                                                workspace.set_groups(private.channels);
                                                workspace.set_ims(im.channels);
                                                workspace.set_mpims(mpim.channels);
                                            }).map_err(|err| {
                                                println!("Error in handle_hello: {:?}", err)
                                            });
                                        tokio::spawn(f);
                                    }
                                };
                                Ok(())
                            });

                            let tx1 = tx.clone();
                            let g = stream
                                .for_each(move |message| {
                                    rtm::handle_message(tx1.clone(), message).unwrap();
                                    Ok(())
                                }).map_err(|err| println!("failed to handle message: {:?}", err));

                            let h = rtm::ping_timer(tx.clone());

                            f.join3(g, h).map_err(|err| println!("{:?}", err))
                        })
                }),
        );
    }

    let f = futures::future::join_all(fs)
        .map(|_| ())
        .map_err(|err| println!("Error: {:?}", err));
    tokio::run(f);
}
