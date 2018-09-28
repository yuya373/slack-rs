mod ping;
pub use self::ping::timer as ping_timer;
pub use tungstenite::{Error, Message};

mod handler;
pub use self::handler::handle_message;

use futures::stream::SplitSink;
use tokio;
use tokio_tls;
use tokio_tungstenite;

pub type Sender = SplitSink<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::stream::Stream<
            tokio::net::TcpStream,
            tokio_tls::TlsStream<tokio::net::TcpStream>,
        >,
    >,
>;
