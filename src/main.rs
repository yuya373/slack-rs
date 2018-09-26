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
    // let client = Arc::new(Client::new());
    // let workspace = config.workspaces[0];

    let f = futures::future::ok(config.workspaces).map(move |workspaces| {
        let client = Client::new();

        for mut workspace in workspaces {
            let client = client.clone();
            let (tx, rx) = mpsc::unbounded::<Action>();

            let f = api::rtm::connect(&workspace, client.clone())
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

                            match action.Type {
                                ActionType::Ping => {
                                    let mut workspace =
                                        workspace.lock().expect("failed to lock workspace 78");
                                    workspace.ping(&sender);

                                    println!("TEAM: {}", workspace.team_name());
                                    println!("public_channels: {:?}", workspace.channels.len());
                                    println!("private_channels: {:?}", workspace.groups.len());
                                }
                                ActionType::Hello => {
                                    let token = workspace
                                        .lock()
                                        .expect("failed to lock workspace 83")
                                        .token
                                        .clone();
                                    let name = workspace.lock().unwrap().team_name();
                                    println!("Receive Hello: {}", name);

                                    use api::conversations::{get_list, ListType};

                                    let public_channels = get_list::<Channel>(
                                        client.clone(),
                                        token.clone(),
                                        ListType::Public,
                                        String::from(""),
                                    ).map_err(|err| {
                                        println!("Error in public_channels: {:?}", err);
                                    });

                                    let private_channels = get_list::<Group>(
                                        client.clone(),
                                        token.clone(),
                                        ListType::Private,
                                        String::from(""),
                                    ).map_err(|err| {
                                        println!("Error in private_channels: {:?}", err);
                                    });

                                    let f = public_channels.join(private_channels).map(
                                        move |(public, private)| {
                                            let mut workspace = workspace.lock().unwrap();
                                            workspace.set_channels(public.channels);
                                            workspace.set_groups(private.channels);

                                            println!("TEAM: {}", name);
                                            println!(
                                                "finished public_channels: {:?}",
                                                workspace.channels.len()
                                            );
                                            println!(
                                                "finished private_channels: {:?}",
                                                workspace.groups.len()
                                            );
                                        },
                                    );
                                    tokio::spawn(f);
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
