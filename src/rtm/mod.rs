extern crate ws;

use super::{Action, ActionType};
use ws::util::Token;
use ws::{CloseCode, Error, ErrorKind, Factory, Handler, Handshake, Message, Result, Sender};

pub struct Client {
    out: Sender,
    tx: super::Tx,
}

impl Client {
    fn new(out: Sender, tx: super::Tx) -> Client {
        Client { out, tx }
    }
}

const PING: Token = Token(0);
impl Handler for Client {
    fn on_open(&mut self, _shake: Handshake) -> Result<()> {
        // println!("Handshake: {:?}", shake);
        println!("Connection Opened");
        self.out.timeout(5000, PING)?;
        Ok(())
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        println!("→ Incoming:    {:?}", message);
        self.tx
            .unbounded_send(Action {
                t: ActionType::Hello,
            }).unwrap();
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The server is done with the connection."),
            CloseCode::Away => println!("The server is leaving"),
            _ => println!(
                "The server encountered an error: {:?} -> {:?}",
                code, reason
            ),
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("Error in rtm: {:?}", err);
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        match event {
            PING => {
                self.tx.unbounded_send(Action::ping()).unwrap();
                self.out.timeout(5000, PING)
            }
            _ => Err(Error::new(ErrorKind::Internal, "Invalid timeout token")),
        }
    }

    fn on_shutdown(&mut self) {
        println!("Client.on_shutdown ");
    }
}

pub struct Connection {
    tx: super::Tx,
}
impl Connection {
    pub fn new(tx: super::Tx) -> Connection {
        Connection { tx }
    }
}
impl Factory for Connection {
    type Handler = Client;

    fn connection_made(&mut self, out: Sender) -> Self::Handler {
        Client::new(out, self.tx.clone())
    }

    fn client_connected(&mut self, out: Sender) -> Self::Handler {
        Client::new(out, self.tx.clone())
    }

    fn connection_lost(&mut self, _: Self::Handler) {
        println!("Connection.connection_lost");
    }

    fn on_shutdown(&mut self) {
        println!("Connection.on_shutdown");
    }
}

// pub fn connect(url: &str, tx: super::Tx) -> Result<&mut WebSocket<Connection>> {
//     let conn = ws::Builder::new().build(Connection { tx })?;
//     conn.connect(Url::parse(url).unwrap())
//     // let handle = conn.broadcaster();
//     // conn.run();
//     // Ok(handle)
//     // ws::connect(url, |out| Client::new(out, tx.clone())).unwrap()
// }
