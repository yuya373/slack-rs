use api::conversations::ListResponse;
use futures::future;
use futures::{Async, Future, Sink};
use model::{Channel, Group, Im, Me, Mpim, Team};
use reqwest;
use reqwest::async::Client;
use rtm::{Message, Sender};
use tungstenite;

#[derive(Deserialize)]
pub struct Workspace {
    pub team: Option<Team>,
    me: Option<Me>,
    pub token: String,
    #[serde(default = "Workspace::init_message_id")]
    message_id: u64,
    #[serde(default = "Workspace::init_channels")]
    pub channels: Vec<Channel>,
    #[serde(default = "Workspace::init_groups")]
    pub groups: Vec<Group>,
    #[serde(default = "Workspace::init_ims")]
    pub ims: Vec<Im>,
    #[serde(default = "Workspace::init_mpims")]
    pub mpims: Vec<Mpim>,
}
impl Workspace {
    fn init_mpims() -> Vec<Mpim> {
        Vec::new()
    }

    fn init_ims() -> Vec<Im> {
        Vec::new()
    }

    fn init_groups() -> Vec<Group> {
        Vec::new()
    }

    fn init_channels() -> Vec<Channel> {
        Vec::new()
    }

    fn init_message_id() -> u64 {
        0
    }

    pub fn team_name(&self) -> String {
        match &self.team {
            Some(team) => team.name.clone(),
            None => String::from(""),
        }
    }

    pub fn set_groups(&mut self, groups: Vec<Group>) {
        self.groups = groups
    }

    pub fn set_channels(&mut self, channels: Vec<Channel>) {
        self.channels = channels;
    }

    pub fn set_ims(&mut self, ims: Vec<Im>) {
        self.ims = ims;
    }

    pub fn set_mpims(&mut self, mpims: Vec<Mpim>) {
        self.mpims = mpims;
    }

    pub fn set_team(&mut self, team: Team) {
        self.team = Some(team);
    }

    pub fn set_me(&mut self, me: Me) {
        self.me = Some(me);
    }

    fn message_id(&mut self) -> u64 {
        let v = self.message_id;
        self.message_id += 1;
        v
    }

    pub fn ping(&mut self) -> Message {
        let id = self.message_id();
        let ping = format!("{{\"id\": \"{id}\", \"type\": \"ping\"}}", id = id);
        ping.into()
    }

    pub fn handle_ping(
        sender: &mut Sender,
        workspace: &mut Self,
    ) -> Result<Async<()>, tungstenite::Error> {
        println!("TEAM: {}", workspace.team_name());
        println!("public_channels: {:?}", workspace.channels.len());
        println!("private_channels: {:?}", workspace.groups.len());
        println!("ims: {:?}", workspace.ims.len());
        println!("mpims: {:?}", workspace.mpims.len());
        sender.start_send(workspace.ping())?;
        sender.poll_complete()
    }

    pub fn handle_hello(
        token: String,
        client: Client,
    ) -> impl Future<
        Item = (
            ListResponse<Channel>,
            ListResponse<Group>,
            ListResponse<Im>,
            ListResponse<Mpim>,
        ),
        Error = reqwest::Error,
    > {
        use api::conversations::{list, ListType};
        let public_channels = list::<Channel>(client.clone(), token.clone(), ListType::Public, "");
        let private_channels = list::<Group>(client.clone(), token.clone(), ListType::Private, "");
        let ims = list::<Im>(client.clone(), token.clone(), ListType::Im, "");
        let mpims = list::<Mpim>(client.clone(), token.clone(), ListType::Mpim, "");
        public_channels
            .join(private_channels)
            .join(ims)
            .join(mpims)
            .map(|(((public, private), im), mpim)| (public, private, im, mpim))
    }
}
