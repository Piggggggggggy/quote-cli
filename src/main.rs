use std::{io::stdout, time::Duration};

use crossterm::event::{poll, read, Event};
use quote_cli;

fn main() {
    poll(Duration::from_secs(2)).unwrap();
    quote_cli::run();
}
