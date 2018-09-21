extern crate ws;
use api::RtmConnectResponse;
use ws::util::Token;
use ws::{CloseCode, Error, ErrorKind, Handler, Handshake, Message, Result, Sender};

struct Client {
    out: Sender,
    message_id: u64,
}

impl Client {
    fn new(out: Sender) -> Client {
        Client { out, message_id: 0 }
    }

    fn send(&mut self, message: Message) -> Result<()> {
        self.message_id += 1;
        println!("← Outgoing:    {:?}", message);
        self.out.send(message)
    }

    fn ping(&mut self) -> Result<()> {
        let id = self.message_id;
        let ping = format!("{{\"id\": \"{id}\", \"type\": \"ping\"}}", id = id);
        self.send(ping.into())
    }
}

const PING: Token = Token(0);
impl Handler for Client {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("Handshake: {:?}", shake);
        println!("Connection Opened");
        self.out.timeout(5000, PING)?;
        Ok(())
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        println!("→ Incoming:    {:?}", message);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The server is done with the connection."),
            CloseCode::Away => println!("The server is leaving"),
            _ => println!("The server encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("Error in rtm: {}", err);
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        match event {
            PING => {
                self.ping()?;
                self.out.timeout(5000, PING)
            }
            _ => Err(Error::new(ErrorKind::Internal, "Invalid timeout token")),
        }
    }
}

pub fn connect(response: &RtmConnectResponse) {
    match &response.url {
        Some(url) => ws::connect(url.clone(), |out| Client::new(out)).unwrap(),
        None => println!("Invalid Connect Response: {:?}", response),
    };
}
