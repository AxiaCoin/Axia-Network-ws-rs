/// WebSocket server used for testing the bench example.
extern crate axia_ws as ws;

use ws::{Builder, Sender, Settings};

fn main() {
    let mut settings = Settings::default();
    settings.max_connections = 10_000;

    Builder::new()
        .with_settings(settings)
        .build(|out: Sender| move |msg| out.send(msg))
        .unwrap()
        .listen("127.0.0.1:3012")
        .unwrap();
}
