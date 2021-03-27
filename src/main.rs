mod render;
mod db;
mod ark;

use crate::db::{add_ark_server_mod_to_db, remove_ark_server_mod_at_index, read_db, add_ark_server_to_db, remove_ark_server_at_index};
use crate::ark::{Event, MenuItem};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, ListState, Paragraph, Tabs, TableState,
    },
    Terminal,
};


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

    let mut editing_mod = false;
    let mut editing_server = false;
    let mut tmp_mod_field = "".to_string();
    let mut tmp_server_field = "".to_string();
    let mut menu_titles = vec!["Home", "Servers", "Quit"];
    let mut active_menu_item = MenuItem::Home;
    let mut active_menu_highlight = MenuItem::Home;

    let mut ark_server_list_state = ListState::default();
    ark_server_list_state.select(Some(0));

    let mut ark_server_list_edit_state = ListState::default();
    ark_server_list_edit_state.select(Some(0));

    let mut ark_server_mod_list_state = ListState::default();
    ark_server_mod_list_state.select(Some(0));

    let mut ark_server_mod_list_edit_state = TableState::default();
    ark_server_mod_list_edit_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = Paragraph::new("ark_server_manager-CLI 2021 - all rights reserved")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_highlight.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => {
                    menu_titles = vec!["Home", "Servers", "Quit"];
                    active_menu_highlight = MenuItem::Home;
                    rect.render_widget(render::home(), chunks[1]);
                }
                MenuItem::Servers => {
                    menu_titles = vec!["Home", "Servers", "Add", "Delete", "Quit"];
                    active_menu_highlight = MenuItem::Servers;
                    let ark_servers_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render::ark_servers(&ark_server_list_state);
                    rect.render_stateful_widget(left, ark_servers_chunks[0], &mut ark_server_list_state);
                    rect.render_widget(right, ark_servers_chunks[1]);
                }
                MenuItem::ViewServer => {
                    menu_titles = vec!["Home", "Servers", "Mods", "Back", "Quit"];
                    active_menu_highlight = MenuItem::Servers;
                    rect.render_widget(render::view_ark_server(&ark_server_list_state), chunks[1]);
                }
                MenuItem::ServerMods => {
                    menu_titles = vec!["Home", "Servers", "Mods", "Add", "Delete", "Back", "Quit"];
                    active_menu_highlight = MenuItem::ServerMods;
                    let ark_servers_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render::ark_server_mods(&ark_server_list_state, &ark_server_mod_list_state);
                    rect.render_stateful_widget(left, ark_servers_chunks[0], &mut ark_server_mod_list_state);
                    rect.render_widget(right, ark_servers_chunks[1]);
                }
                MenuItem::ViewMod => {
                    menu_titles = vec!["Home", "Servers", "Mods", "Toggle", "Edit", "Back", "Quit"];
                    active_menu_highlight = MenuItem::ServerMods;
                    rect.render_widget(render::view_ark_server_mod(&ark_server_list_state, &ark_server_mod_list_state), chunks[1]);
                }
                MenuItem::EditMod => {
                    menu_titles = vec!["Home", "Servers", "Mods", "Back", "Quit"];
                    active_menu_highlight = MenuItem::ServerMods;
                    let left = render::edit_ark_server_mod(&ark_server_list_state, &ark_server_mod_list_state);
                    rect.render_stateful_widget(left, chunks[1], &mut ark_server_mod_list_edit_state);
                }
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

        if editing_server {
            match rx.recv()? {
                ark::Event::Input(event) => match event.code {
                    KeyCode::Enter => {
                        editing_server = false;
                    }
                    KeyCode::Backspace => {
                    }
                    _ => {
                        tmp_server_field += get_input_char(event.code);
                    }
                },
                ark::Event::Tick => {}
            }
        } else if editing_mod {
            match rx.recv()? {
                ark::Event::Input(event) => match event.code {
                    KeyCode::Enter => {
                        editing_mod = false;
                    }
                    _ => {
                        tmp_mod_field += get_input_char(event.code);
                    }
                },
                ark::Event::Tick => {}
            }

        } else {
            match rx.recv()? {
                ark::Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                    KeyCode::Char('s') => active_menu_item = MenuItem::Servers,
                    _ => {
                        match active_menu_item {
                            MenuItem::Home => {
                                match event.code {
                                    _ => {}
                                }
                            }
                            MenuItem::ViewServer => {
                                match event.code {
                                    KeyCode::Char('m') => {
                                        ark_server_mod_list_state.select(Some(0));
                                        active_menu_item = MenuItem::ServerMods
                                    }
                                    KeyCode::Char('b') => {
                                        active_menu_item = MenuItem::Servers
                                    }
                                    _ => {}
                                }
                            }
                            MenuItem::ViewMod => {
                                match event.code {
                                    KeyCode::Char('b') => {
                                        active_menu_item = MenuItem::ServerMods
                                    }
                                    KeyCode::Char('t') => {
                                        //TODO Toggle server mod
                                    }
                                    KeyCode::Char('e') => {
                                        active_menu_item = MenuItem::EditMod
                                    }
                                    _ => {}
                                }
                            }
                            MenuItem::EditMod => {
                                match event.code {
                                    KeyCode::Char('b') => {
                                        active_menu_item = MenuItem::ViewMod
                                    }
                                    KeyCode::Enter => {
                                        editing_mod = true;
                                    }
                                    KeyCode::Down => {
                                        if let Some(selected) = ark_server_mod_list_edit_state.selected() {
                                            //This magic number needs to match field count - 1
                                            if selected < 4 {
                                                ark_server_mod_list_edit_state.select(Some(selected + 1));
                                            }
                                        }
                                    }
                                    KeyCode::Up => {
                                        if let Some(selected) = ark_server_mod_list_edit_state.selected() {
                                            if selected > 0 {
                                                ark_server_mod_list_edit_state.select(Some(selected - 1));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            MenuItem::ServerMods => {
                                match event.code {
                                    KeyCode::Char('a') => {
                                        add_ark_server_mod_to_db(&ark_server_list_state).expect("can add new random ark_server");
                                    }
                                    KeyCode::Char('d') => {
                                        remove_ark_server_mod_at_index(&mut ark_server_list_state, &mut ark_server_mod_list_state).expect("can remove ark_server mod");
                                    }
                                    KeyCode::Char('b') => {
                                        active_menu_item = MenuItem::ViewServer
                                    }
                                    KeyCode::Enter => {
                                        active_menu_item = MenuItem::ViewMod;
                                    }
                                    KeyCode::Down => {
                                        if let Some(selected) = ark_server_mod_list_state.selected() {
                                            let ark_server_list = read_db().expect("can fetch ark_server list");
                                            let selected_ark_server = ark_server_list
                                                .get(
                                                    ark_server_list_state
                                                    .selected()
                                                    .expect("there is always a selected ark_server"),
                                                    )
                                                .expect("exists")
                                                .clone();
                                            let amount_ark_server_mods = selected_ark_server.mods.len();
                                            if amount_ark_server_mods < 1 {
                                                ark_server_mod_list_state.select(Some(0));
                                            } else if selected >= amount_ark_server_mods - 1 {
                                                ark_server_mod_list_state.select(Some(0));
                                            } else {
                                                ark_server_mod_list_state.select(Some(selected + 1));
                                            }
                                        }
                                    }
                                    KeyCode::Up => {
                                        if let Some(selected) = ark_server_mod_list_state.selected() {
                                            let ark_server_list = read_db().expect("can fetch ark_server list");
                                            let selected_ark_server = ark_server_list
                                                .get(
                                                    ark_server_list_state
                                                    .selected()
                                                    .expect("there is always a selected ark_server"),
                                                    )
                                                .expect("exists")
                                                .clone();
                                            let amount_ark_server_mods = selected_ark_server.mods.len();
                                            if amount_ark_server_mods < 1 {
                                                ark_server_mod_list_state.select(Some(0));
                                            } else if selected > 0 {
                                                ark_server_mod_list_state.select(Some(selected - 1));
                                            } else {
                                                ark_server_mod_list_state.select(Some(amount_ark_server_mods - 1));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            MenuItem::Servers => {
                                match event.code {
                                    KeyCode::Char('a') => {
                                        add_ark_server_to_db().expect("can add new random ark_server");
                                    }
                                    KeyCode::Char('d') => {
                                        remove_ark_server_at_index(&mut ark_server_list_state).expect("can remove ark_server");
                                    }
                                    KeyCode::Enter => {
                                        active_menu_item = MenuItem::ViewServer;
                                    }
                                    KeyCode::Down => {
                                        if let Some(selected) = ark_server_list_state.selected() {
                                            let amount_ark_servers = read_db().expect("can fetch ark_server list").len();
                                            if amount_ark_servers < 1 {
                                                ark_server_list_state.select(Some(0));
                                            } else if selected >= amount_ark_servers - 1 {
                                                ark_server_list_state.select(Some(0));
                                            } else {
                                                ark_server_list_state.select(Some(selected + 1));
                                            }
                                        }
                                    }
                                    KeyCode::Up => {
                                        if let Some(selected) = ark_server_list_state.selected() {
                                            let amount_ark_servers = read_db().expect("can fetch ark_server list").len();
                                            if amount_ark_servers < 1 {
                                                ark_server_list_state.select(Some(0));
                                            } else if selected > 0 {
                                                ark_server_list_state.select(Some(selected - 1));
                                            } else {
                                                ark_server_list_state.select(Some(amount_ark_servers - 1));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                },
                ark::Event::Tick => {}
            }
        }
    }

    Ok(())
}

fn get_input_char(code: KeyCode) -> &'static str {
    match code {
        KeyCode::Char('a') => return "a",
        KeyCode::Char('b') => return "b",
        _ => return "",
    }
}
