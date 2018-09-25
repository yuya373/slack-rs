use model::{Purpose, Topic, UserId};

#[derive(Debug, Deserialize)]
pub struct Group {
    id: String,
    name: String,
    is_channel: bool,
    is_im: bool,
    created: u64,
    creator: UserId,
    is_archived: bool,
    is_general: bool,
    name_normalized: String,
    is_ext_shared: bool,
    shared_team_ids: Vec<String>,
    is_pending_ext_shared: bool,
    is_shared: bool,
    is_org_shared: bool,
    is_member: bool,
    is_private: bool,
    is_mpim: bool,
    topic: Topic,
    purpose: Purpose,
}
