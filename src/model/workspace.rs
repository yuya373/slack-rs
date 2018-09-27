use model::{Channel, Group, Me, Team};
use rtm::Message;

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
}
impl Workspace {
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
}
