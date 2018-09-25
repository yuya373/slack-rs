use model::UserId;

#[derive(Debug, Deserialize)]
pub struct Me {
    id: UserId,
    name: String,
}
