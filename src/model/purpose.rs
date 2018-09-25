use model::UserId;

#[derive(Debug, Deserialize)]
pub struct Purpose {
    value: String,
    creator: UserId,
    last_set: u64,
}
