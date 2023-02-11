use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use inputbot::{self, KeybdKey::*};
use inquire::MultiSelect;
use once_cell::sync::OnceCell;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::io::stdout;
use std::thread;
use std::vec::IntoIter;
use std::{
    fmt, fs,
    iter::Cycle,
    process::exit,
    sync::{Arc, Mutex},
};
static QUOTE_ITER: OnceCell<Arc<Mutex<Cycle<IntoIter<Quote>>>>> = OnceCell::new();

fn init() -> Arc<Mutex<Cycle<IntoIter<Quote>>>> {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    let dir = "QuoteLists";
    let quote_lists: Vec<QuoteList> = fs::read_dir(dir)
        .unwrap()
        .map(|file| {
            let path = file.unwrap().path();
            if let Ok(file_content) = fs::read_to_string(path) {
                return QuoteList::build(file_content);
            } else {
                eprintln!("could not read file");
                exit(1)
            }
        })
        .collect();
    println!(
        "Press {} to exit; Use {} to paste a new quote.",
        "Esc".dark_blue(),
        "Control + V".dark_blue()
    );
    let quotes =
        match MultiSelect::new("Choose the quote lists you would like to use.", quote_lists)
            .prompt()
        {
            Ok(quote) => quote,
            Err(_) => exit(0),
        };
    let mut all_quotes: Vec<Quote> = Vec::new();

    for quote_list in quotes {
        for quote in quote_list.quotes {
            all_quotes.push(Quote {
                author: quote_list.author.clone(),
                quote,
            })
        }
    }
    // shuffle the quotes
    let mut rng = thread_rng();
    all_quotes.shuffle(&mut rng);
    Arc::new(Mutex::new(all_quotes.into_iter().cycle()))
}

pub fn run() {
    QUOTE_ITER.get_or_init(init);

    VKey.bind(|| {
        if LControlKey.is_pressed() || RControlKey.is_pressed() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            if let Some(quote) = QUOTE_ITER.wait().lock().unwrap().next() {
                ctx.set_contents(format!("{} -{}", quote.quote.clone(), quote.author.clone()))
                    .unwrap();
                println!("{} -{}", quote.quote.clone(), quote.author.clone());
            } else {
                exit(0)
            }
        }
    });
    thread::spawn(|| loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key) if key.code == KeyCode::Esc => {
                exit(0);
            }
            _ => (),
        }
    });

    inputbot::handle_input_events();
}
#[derive(Clone)]
struct Quote {
    author: String,
    quote: String,
}

struct QuoteList {
    author: String,
    quotes: Vec<String>,
}

impl fmt::Display for QuoteList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.author)
    }
}

impl QuoteList {
    fn build(s: String) -> QuoteList {
        // todo make this more robust
        let author = s.lines().next().unwrap().trim().replace("Author: ", "");
        let quotes = s.lines().skip(1).map(|line| line.to_string()).collect();
        QuoteList { author, quotes }
    }
}
