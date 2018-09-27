extern crate dirs;
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate tokio_timer;
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
                                let sender = sender.clone();

                                match action.Type {
                                    ActionType::Ping => {
                                        let mut workspace =
                                            workspace.lock().expect("failed to lock workspace 78");

                                        println!("TEAM: {}", workspace.team_name());
                                        println!("public_channels: {:?}", workspace.channels.len());
                                        println!("private_channels: {:?}", workspace.groups.len());
                                        let mut sender = sender.lock().unwrap();
                                        sender.start_send(workspace.ping());
                                        sender.poll_complete();
                                        // futures::future::poll_fn(|| sender.poll_complete())
                                    }
                                    ActionType::Hello => {
                                        let token = workspace
                                            .lock()
                                            .expect("failed to lock workspace 83")
                                            .token
                                            .clone();
                                        let name = workspace.lock().unwrap().team_name();
                                        println!("Receive Hello: {}", name);

                                        use api::conversations::{list, ListType};

                                        let public_channels = list::<Channel>(
                                            client.clone(),
                                            token.clone(),
                                            ListType::Public,
                                            String::from(""),
                                        ).map_err(|err| {
                                            println!("Error in public_channels: {:?}", err);
                                        });

                                        let private_channels = list::<Group>(
                                            client.clone(),
                                            token.clone(),
                                            ListType::Private,
                                            String::from(""),
                                        ).map_err(|err| {
                                            println!("Error in private_channels: {:?}", err);
                                        });

                                        let f = public_channels
                                            .join(private_channels)
                                            .map_err(|err| {
                                                println!("Failed to list channels: {:?}", err)
                                            }).map(
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

                            #[derive(Deserialize, Debug)]
                            struct MessageType<'a> {
                                #[serde(rename = "type")]
                                Type: &'a str,
                            }

                            let tx1 = tx.clone();
                            let g = stream
                                .for_each(move |message| {
                                    let m: MessageType =
                                        serde_json::from_str(message.to_text().unwrap()).unwrap();
                                    let tx = &tx1;

                                    match m.Type {
                                        "hello" => tx.unbounded_send(Action::hello()).unwrap(),
                                        _ => {
                                            println!("Receive message: {:?}", m);
                                        }
                                    }
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
