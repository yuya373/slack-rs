use super::Message;
use futures::sync::mpsc;
use serde_json;
use Action;
use Tx;

#[derive(Deserialize, Debug)]
struct MessageType<'a> {
    #[serde(rename = "type")]
    Type: &'a str,
}

pub fn handle_message(tx: Tx, message: Message) -> Result<(), mpsc::SendError<Action>> {
    let m: MessageType = serde_json::from_str(message.to_text().unwrap()).unwrap();

    match m.Type {
        "hello" => tx.unbounded_send(Action::hello()),
        _ => {
            println!("Receive message: {:?}", m);
            Ok(())
        }
    }
}
