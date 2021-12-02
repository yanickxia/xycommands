use std::{io, thread};
use std::borrow::BorrowMut;
use std::process::exit;
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

use log::info;
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use termion::event::Event;

pub struct KeyBoard {}

impl KeyBoard {
    pub fn new() -> Self {
        KeyBoard {}
    }
}

impl KeyBoard {
    pub fn run_background(&mut self) {
        let events = KeyBoard::events();
        loop {
            match events.recv().unwrap() {
                Key::Char('q') => {
                    let running = Arc::clone(&crate::state::RUNNING);
                    let mut lock = crate::state::RUNNING.lock().unwrap();
                    *lock = false;
                    break;
                }
                _ => {}
            }
        }
    }

    fn events() -> mpsc::Receiver<Key> {
        let (tx, rx) = mpsc::channel();
        let keys_tx = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    if let Err(err) = keys_tx.send(key) {
                        eprintln!("{}", err);
                        return;
                    }
                }
            }
        });
        rx
    }
}

