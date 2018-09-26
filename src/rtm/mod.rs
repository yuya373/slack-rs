extern crate tokio;
extern crate tokio_tls;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate url;

use super::Tx;
use futures::stream::SplitSink;
use futures::{Future, Stream};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::WebSocketStream;
pub use tungstenite::{Error, Message};

pub type Sender = SplitSink<
    WebSocketStream<
        tokio_tungstenite::stream::Stream<
            tokio::net::TcpStream,
            tokio_tls::TlsStream<tokio::net::TcpStream>,
        >,
    >,
>;

pub fn connect(url: &str, tx: Tx) -> impl Future<Item = Sender, Error = Error> {
    let url = url::Url::parse(url).unwrap();
    connect_async(url).and_then(move |(ws_stream, _)| {
        println!("rtm::connect!");
        let (sink, stream) = ws_stream.split();
        stream.for_each(|message| {
            let s = message.into_text();
            println!("Receive message: {:?}", s);
            Ok(())
        });
        Ok(sink)
    })
}
