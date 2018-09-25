use model::UserId;

#[derive(Debug, Deserialize)]
pub struct Im {
    id: String,
    created: u64,
    is_im: bool,
    is_org_shared: bool,
    user: UserId,
    is_user_deleted: bool,
}
