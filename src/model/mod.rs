use api::RtmConnectResponse;
use futures::Future;
use futures::Stream;
use ws;

#[derive(Deserialize)]
pub struct Workspace {
    team: Option<Team>,
    me: Option<Me>,
    pub token: String,
    ws_url: Option<String>,
    #[serde(skip)]
    rx: Option<super::Rx>,
    #[serde(skip)]
    ws: Option<ws::Sender>,
}
impl Workspace {
    // pub fn new(token: &str) -> Workspace {
    //     Workspace {
    //         token: token.to_string(),
    //         team: None,
    //         me: None,
    //     }
    // }
    pub fn merge(&mut self, resp: RtmConnectResponse, rx: super::Rx) {
        self.team = resp.team;
        self.me = resp.me;
        self.ws_url = resp.url;
        self.rx = Some(rx);
    }

    pub fn set_ws(&mut self, ws: ws::Sender) {
        self.ws = Some(ws);
    }

    pub fn ws_url(&self) -> String {
        self.ws_url.clone().unwrap()
    }

    pub fn process(self) -> impl Future<Item = (), Error = ()> {
        println!("Process Action");
        let f = |action: super::Action| {
            println!("Receive Action: {:?}", action);
            Ok(())
        };
        self.rx.unwrap().for_each(f)
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
