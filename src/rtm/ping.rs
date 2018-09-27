extern crate tokio_timer;

use futures::{Future, Stream};
use std::time::{Duration, Instant};
use tokio_timer::Interval;
use Action;

pub fn timer(tx: ::Tx) -> impl Future<Item = (), Error = ()> {
    Interval::new(Instant::now(), Duration::from_secs(5))
        .for_each(move |_| {
            tx.unbounded_send(Action::ping())
                .map_err(|err| println!("Failed to send Action::ping, {:?}", err));
            Ok(())
        }).map_err(|err| println!("Failed to ping_timer, {:?}", err))
}
