mod render;
mod db;
mod service;
mod ark;
mod input;

use crate::ark::{Event, ProgState, InputEvent};
use crate::render::{render};
use crate::input::{process_input};

use crossterm::{
    event::{self, Event as CEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{ backend::CrosstermBackend, Terminal };


const DB_PATH: &str = "./data/db.json";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(ark::Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;


    let mut state = ProgState::new();

    loop {
        render(&mut terminal, &mut state).expect("Renders successfully");

        let input = rx.recv().expect("Recieved input");
        let processed = process_input(&mut state, input).expect("Input processed");
        match processed {
            InputEvent::Exit => {
                disable_raw_mode().expect("Raw mode disabled");
                terminal.show_cursor()?;
                terminal.clear()?;
                break
            }
            _ => {}
        }
    }

    Ok(())
}
