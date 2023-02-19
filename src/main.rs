use std::{io::stdout, time::Duration};

use crossterm::event::{poll, read, Event};
use quote_cli;

fn main() {
    quote_cli::run();
}
