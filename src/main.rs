use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

const DB_PATH: &str = "./data/db.json";

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Serialize, Deserialize, Clone)]
struct ArkServerMod {
    id: usize,
    name: String,
    category: String,
    descripton: String,
    enabled: bool,
    age: usize,
    created_at: DateTime<Utc>,
}

impl ArkServerMod {
    fn new() -> ArkServerMod {
        ArkServerMod {
            id: 0,
            name: "".to_string(),
            category: "".to_string(),
            descripton: "".to_string(),
            enabled: false,
            age: 0,
            created_at: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct ArkServer {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
    mods: Vec<ArkServerMod>,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Servers,
    ViewServer,
    ViewServerMods,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Servers => 1,
            MenuItem::ViewServerMods => 2,
            MenuItem::ViewServer => 3,
        }
    }
}

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
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut menu_titles = vec!["Home", "Servers", "Quit"];
    let mut active_menu_item = MenuItem::Home;
    let mut active_menu_highlight = MenuItem::Home;
    let mut ark_server_list_state = ListState::default();
    ark_server_list_state.select(Some(0));
    let mut ark_server_mod_list_state = ListState::default();
    ark_server_mod_list_state.select(Some(0));

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
                    rect.render_widget(render_home(), chunks[1]);
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
                    let (left, right) = render_ark_servers(&ark_server_list_state);
                    rect.render_stateful_widget(left, ark_servers_chunks[0], &mut ark_server_list_state);
                    rect.render_widget(right, ark_servers_chunks[1]);
                }
                MenuItem::ViewServer => {
                    menu_titles = vec!["Home", "Servers", "Mods", "Back", "Quit"];
                    active_menu_highlight = MenuItem::Servers;
                    rect.render_widget(render_view_ark_server(&ark_server_list_state), chunks[1]);
                }
                MenuItem::ViewServerMods => {
                    menu_titles = vec!["Home", "Servers", "Mods", "Add", "Delete", "Back", "Quit"];
                    active_menu_highlight = MenuItem::ViewServerMods;
                    let ark_servers_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_ark_server_mods(&ark_server_list_state, &ark_server_mod_list_state);
                    rect.render_stateful_widget(left, ark_servers_chunks[0], &mut ark_server_mod_list_state);
                    rect.render_widget(right, ark_servers_chunks[1]);
                }
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
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
                                    active_menu_item = MenuItem::ViewServerMods
                                }
                                KeyCode::Char('b') => {
                                    active_menu_item = MenuItem::Servers
                                }
                                _ => {}
                            }
                        }
                        MenuItem::ViewServerMods => {
                            match event.code {
                                KeyCode::Char('a') => {
                                    add_ark_server_mod_to_db().expect("can add new random ark_server");
                                }
                                KeyCode::Char('d') => {
                                    remove_ark_server_mod_at_index(&mut ark_server_list_state, &mut ark_server_mod_list_state).expect("can remove ark_server mod");
                                }
                                KeyCode::Char('b') => {
                                    active_menu_item = MenuItem::ViewServer
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
                                    if let Some(selected) = ark_server_list_state.selected() {
                                        let mut ark_servers = read_db().expect("can fetch ark_server list");
                                        let mut server = ark_servers.pop().unwrap();
                                        active_menu_item = MenuItem::ViewServer;
                                    }
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
                        _ => {}
                    }
                }
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "ark_server-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 's' to access ark_servers, 'a' to add random new ark_servers and 'd' to delete the currently selected ark_server.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

fn render_view_ark_server<'a>(ark_server_list_state: &ListState) -> Table<'a> {
    let ark_servers = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Servers")
        .border_type(BorderType::Plain);

    let ark_server_list = read_db().expect("can fetch ark_server list");
    let items: Vec<_> = ark_server_list
        .iter()
        .map(|ark_server| {
            ListItem::new(Spans::from(vec![Span::styled(
                ark_server.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_ark_server = ark_server_list
        .get(
            ark_server_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let mods_str = selected_ark_server.mods.into_iter().map(|i| i.name + &", ".to_string()).collect::<String>();

    let ark_server_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_ark_server.id.to_string())),
        Cell::from(Span::raw(selected_ark_server.name)),
        Cell::from(Span::raw(selected_ark_server.category)),
        Cell::from(Span::raw(selected_ark_server.age.to_string())),
        Cell::from(Span::raw(selected_ark_server.created_at.to_string())),
        Cell::from(Span::raw(mods_str)),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Mods",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    ark_server_detail
}

fn render_ark_server_mods<'a>(ark_server_list_state: &ListState, ark_server_mod_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let ark_server_mods = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Server Mods")
        .border_type(BorderType::Plain);

    let ark_server_list = read_db().expect("can fetch ark_server list");

    let selected_ark_server = ark_server_list
        .get(
            ark_server_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let items: Vec<_> = selected_ark_server.mods
        .iter()
        .map(|ark_server| {
            ListItem::new(Spans::from(vec![Span::styled(
                ark_server.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items).block(ark_server_mods).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );


    let selected_ark_server_mod = if selected_ark_server.mods.len() > 0 {
        selected_ark_server.mods
            .get(
                ark_server_mod_list_state
                    .selected()
                    .expect("there is always a selected ark_server"),
            )
            .expect("exists")
            .clone()
    } else { ArkServerMod::new() };

    let ark_server_mod_detail = if selected_ark_server_mod.id != 0 {
        Table::new(vec![Row::new(vec![
            Cell::from(Span::raw(selected_ark_server_mod.id.to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.name)),
            Cell::from(Span::raw(selected_ark_server_mod.category)),
            Cell::from(Span::raw(selected_ark_server_mod.age.to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.created_at.to_string())),
        ])])
    } else {
        Table::new(vec![Row::new(vec![
            Cell::from(Span::raw("".to_string())),
            Cell::from(Span::raw("".to_string())),
            Cell::from(Span::raw("".to_string())),
            Cell::from(Span::raw("".to_string())),
            Cell::from(Span::raw("".to_string())),
        ])])
    }
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);
    (list, ark_server_mod_detail)

}

fn render_ark_servers<'a>(ark_server_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let ark_servers = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Servers")
        .border_type(BorderType::Plain);

    let ark_server_list = read_db().expect("can fetch ark_server list");
    let items: Vec<_> = ark_server_list
        .iter()
        .map(|ark_server| {
            ListItem::new(Spans::from(vec![Span::styled(
                ark_server.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_ark_server = ark_server_list
        .get(
            ark_server_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(ark_servers).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    let mods_str = selected_ark_server.mods.len().to_string();

    let ark_server_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_ark_server.id.to_string())),
        Cell::from(Span::raw(selected_ark_server.name)),
        Cell::from(Span::raw(selected_ark_server.category)),
        Cell::from(Span::raw(selected_ark_server.age.to_string())),
        Cell::from(Span::raw(selected_ark_server.created_at.to_string())),
        Cell::from(Span::raw(mods_str)),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Mods",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    (list, ark_server_detail)
}

fn read_db() -> Result<Vec<ArkServer>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn add_ark_server_to_db() -> Result<Vec<ArkServer>, Error> {
    let mut rng = rand::thread_rng();
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;

    let random_ark_server = ArkServer {
        id: rng.gen_range(0, 9999999),
        name: rng.sample_iter(Alphanumeric).take(10).collect(),
        category: "TODO Categories".to_string(),
        age: rng.gen_range(1, 15),
        created_at: Utc::now(),
        mods: Vec::new(),
    };

    parsed.push(random_ark_server);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

fn remove_ark_server_at_index(ark_server_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = ark_server_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        ark_server_list_state.select(Some(selected - 1));
    }
    Ok(())
}

fn add_ark_server_mod_to_db() -> Result<Vec<ArkServer>, Error> {
    let mut rng = rand::thread_rng();
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;

    let random_ark_server_mod = ArkServerMod {
        id: rng.gen_range(0, 9999999),
        name: rng.sample_iter(Alphanumeric).take(10).collect(),
        category: "TODO Categories".to_string(),
        descripton: "TODO Description".to_string(),
        enabled: false,
        age: rng.gen_range(1, 15),
        created_at: Utc::now(),
    };

    let mut server: ArkServer = parsed.pop().unwrap();
    server.mods.push(random_ark_server_mod);

    parsed.push(server);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

fn remove_ark_server_mod_at_index(ark_server_list_state: &mut ListState, ark_server_mod_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected_server) = ark_server_list_state.selected() {
        if let Some(selected_mod) = ark_server_mod_list_state.selected() {

            let db_content = fs::read_to_string(DB_PATH)?;

            let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
            let mut server: ArkServer = parsed.pop().unwrap();

            server.mods.remove(selected_mod);
            parsed.push(server);
            fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
            if selected_mod > 0 {
                ark_server_mod_list_state.select(Some(selected_mod - 1));
            }
        }
    }
    Ok(())
}
