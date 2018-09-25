extern crate ws;

use api::conversations::{list_request, ListResponse, ListType};
use futures::future::{loop_fn, ok, Loop};
use futures::Future;
use model::{Channel, Group, Im, Me, Mpim, Team};
use reqwest;
use reqwest::async::Client;
use serde;
use serde_json;
use std::sync::Arc;
use tokio::spawn;
use ws::{Message, Result, Sender};

#[derive(Deserialize)]
pub struct Workspace {
    pub team: Option<Team>,
    me: Option<Me>,
    pub token: String,
    #[serde(default = "Workspace::init_message_id")]
    message_id: u64,
    #[serde(default = "Workspace::init_channels")]
    pub channels: Vec<Channel>,
}
impl Workspace {
    // pub fn new(token: &str) -> Workspace {
    //     Workspace {
    //         token: token.to_string(),
    //         team: None,
    //         me: None,
    //     }
    // }
    fn init_channels() -> Vec<Channel> {
        Vec::new()
    }
    fn init_message_id() -> u64 {
        0
    }

    pub fn team_name(&self) -> &str {
        match &self.team {
            Some(team) => &team.name,
            None => "",
        }
    }

    pub fn set_channels(&mut self, channels: Vec<Channel>) {
        self.channels = channels;
    }

    pub fn set_team(&mut self, team: Team) {
        self.team = Some(team);
    }

    pub fn set_me(&mut self, me: Me) {
        self.me = Some(me);
    }

    fn send(&mut self, sender: &Sender, message: Message) -> Result<()> {
        self.message_id += 1;
        println!("← Outgoing:    {:?}", message);
        sender.send(message)
    }

    pub fn ping(&mut self, sender: &Sender) {
        let id = self.message_id;
        let ping = format!("{{\"id\": \"{id}\", \"type\": \"ping\"}}", id = id);
        self.send(sender, ping.into()).unwrap();
    }

    pub fn list_conversations<T>(
        &self,
        client: &Client,
        types: ListType,
        cursor: Option<&str>,
    ) -> impl Future<Item = ListResponse<T>, Error = reqwest::Error>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        list_request(&self.token, client, types, cursor)
            .send()
            .and_then(|mut res| {
                // println!("res: {:?}", res.json::<serde_json::Value>());
                ok(res)
            }).and_then(|mut res| res.json::<ListResponse<T>>())
    }

    // pub fn hello(&self, client: &Client) {
    //     println!("Connected to Slack!");

    //     // let token = &self.token.clone();
    //     // let client = *client;

    //     let public_channels = loop_fn(ListResponse::empty(), move |mut res| {
    //         let next_cursor = res.response_metadata.next_cursor.clone();

    //         list_request(
    //             self.token.as_str(),
    //             client,
    //             ListType::Public,
    //             Some(&next_cursor),
    //         ).send()
    //         .and_then(|mut res| res.json::<ListResponse<Channel>>())
    //         .and_then(|mut new_res| {
    //             res.channels.append(&mut new_res.channels);
    //             res.response_metadata = new_res.response_metadata;
    //             res.ok = new_res.ok;

    //             if res.ok && res.response_metadata.next_cursor.len() > 0 {
    //                 Ok(Loop::Continue(res))
    //             } else {
    //                 Ok(Loop::Continue(res))
    //             }
    //         }).map_err(|err| println!("Error in public_channels: {:?}", err))
    //     });
    //     spawn(public_channels);

    //     // let public_channels = self
    //     //     .list_conversations::<Channel>(client, ListType::Public, None)
    //     //     .map(|res| {
    //     //         println!("public_channels: {:?}", res.channels.len());
    //     //     }).map_err(|err| println!("Error in conversations.list:public: {:?}", err));
    //     // spawn(public_channels);

    //     // let private_channels = self
    //     //     .list_conversations::<Group>(client, ListType::Private, None)
    //     //     .map(|res| {
    //     //         println!("private_channels: {:?}", res.channels.len());
    //     //     }).map_err(|err| println!("Error in conversations.list:private: {:?}", err));
    //     // spawn(private_channels);

    //     // let ims = self
    //     //     .list_conversations::<Im>(client, ListType::Im, None)
    //     //     .map(|res| {
    //     //         println!("ims: {:?}", res.channels.len());
    //     //     }).map_err(|err| println!("Error in conversations.list:im: {:?}", err));

    //     // spawn(ims);
    //     // let mpims = self
    //     //     .list_conversations::<Mpim>(client, ListType::Mpim, None)
    //     //     .map(|res| {
    //     //         println!("mpims: {:?}", res.channels.len());
    //     //     }).map_err(|err| println!("Error in conversations.list:mpim: {:?}", err));
    //     // spawn(mpims);
    // }
}
