use futures::Future;
use reqwest::async::Client;
use serde_json;
use tokio::spawn;
use ws::{Message, Result, Sender};

#[derive(Deserialize)]
pub struct Workspace {
    team: Option<Team>,
    me: Option<Me>,
    pub token: String,
    #[serde(default = "Workspace::init_message_id")]
    message_id: u64,
}
impl Workspace {
    // pub fn new(token: &str) -> Workspace {
    //     Workspace {
    //         token: token.to_string(),
    //         team: None,
    //         me: None,
    //     }
    // }
    fn init_message_id() -> u64 {
        0
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

    pub fn hello(&mut self, client: &Client) {
        use api::{conversations_list_request, ConversationsListTypes};
        println!("Connected to Slack!");
        let f = conversations_list_request(&self.token, client, ConversationsListTypes::All, None)
            .send()
            .and_then(|mut res| res.json::<serde_json::Value>())
            .map(|res| {
                println!("conversations.list: {:?}", res);
            }).map_err(|err| println!("Error in conversations.list: {:?}", err));
        spawn(f);
    }
}
#[derive(Debug, Deserialize)]
pub struct Team {
    id: String,
    name: String,
    domain: String,
}
#[derive(Debug, Deserialize)]
pub struct Me {
    id: String,
    name: String,
}

type UserId = String;
type Timestamp = String;

#[derive(Debug, Deserialize)]
struct Channel {
    id: String,
    name: String,
    is_channel: bool,
    created: u64,
    creater: UserId,
    is_archived: bool,
    is_general: bool,
    name_normalized: String,
    is_shared: bool,
    is_org_shared: bool,
    is_member: bool,
    is_private: bool,
    is_mpim: bool,
    last_read: Timestamp,
    unread_count: u64,
    unread_count_display: u64,
    members: Vec<UserId>,
    topic: Topic,
    purpose: Purpose,
    previous_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Topic {
    value: String,
    creator: UserId,
    last_set: u64,
}

#[derive(Debug, Deserialize)]
struct Purpose {
    value: String,
    creator: UserId,
    last_set: u64,
}
