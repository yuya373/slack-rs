use model::UserId;

#[derive(Debug, Deserialize)]
pub struct Topic {
    value: String,
    creator: UserId,
    last_set: u64,
}
