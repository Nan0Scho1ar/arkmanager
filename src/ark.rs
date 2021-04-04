use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
    #[error("error, invalid selection")]
    SelectionError,
    #[error("error, rendering failed")]
    RenderError,
}

pub enum Event<I> {
    Input(I),
    Tick,
}

pub enum InputEvent {
    Exit,
    Other,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArkServerMod {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub descripton: String,
    pub enabled: bool,
    pub age: usize,
    pub created_at: DateTime<Utc>,
}

impl ArkServerMod {
    pub fn named(name: &str) -> ArkServerMod {
        ArkServerMod {
            id: 0,
            name: name.to_string(),
            category: "".to_string(),
            descripton: "".to_string(),
            enabled: false,
            age: 0,
            created_at: Utc::now(),
        }
    }
    pub fn new() -> ArkServerMod {
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
pub struct ArkServer {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub age: usize,
    pub created_at: DateTime<Utc>,
    pub mods: Vec<ArkServerMod>,
}

impl ArkServer {
    pub fn named(name: &str) -> ArkServer {
        ArkServer {
            id: 0,
            name: name.to_string(),
            category: "".to_string(),
            age: 0,
            created_at: Utc::now(),
            mods: Vec::new(),
        }
    }
    pub fn new() -> ArkServer {
        ArkServer {
            id: 0,
            name: "".to_string(),
            category: "".to_string(),
            age: 0,
            created_at: Utc::now(),
            mods: Vec::new(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Servers,
    ViewServer,
    ServerMods,
    ViewMod,
    EditMod,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Servers => 1,
            MenuItem::ServerMods => 2,
            MenuItem::ViewServer => 3,
            MenuItem::ViewMod => 4,
            MenuItem::EditMod => 5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RenderState<'a> {
    pub editing_mod: bool,
    pub editing_server: bool,
    pub tmp_mod_field: String,
    pub tmp_server_field: String,
    pub menu_titles: Vec<&'a str>,
    pub active_menu_item: MenuItem,
    pub active_menu_highlight: MenuItem,
    pub ark_server_list_state: ListState,
    pub ark_server_list_edit_state: ListState,
    pub ark_server_mod_list_state: ListState,
    pub ark_server_mod_list_edit_state: TableState,
    pub num_ark_server_mod_properties: usize,
}

impl<'a> RenderState<'a> {
    pub fn new() -> RenderState<'a> {
        let mut rs = RenderState {
             editing_mod: false,
             editing_server: false,
             tmp_mod_field: "".to_string(),
             tmp_server_field: "".to_string(),
             menu_titles: vec!["Home", "Servers", "Quit"],
             active_menu_item: MenuItem::Home,
             active_menu_highlight: MenuItem::Home,
             ark_server_list_state: ListState::default(),
             ark_server_list_edit_state: ListState::default(),
             ark_server_mod_list_state: ListState::default(),
             ark_server_mod_list_edit_state: TableState::default(),
             num_ark_server_mod_properties: 5,
        };
        rs.ark_server_list_state.select(Some(0));
        rs.ark_server_list_edit_state.select(Some(0));
        rs.ark_server_mod_list_state.select(Some(0));
        rs.ark_server_mod_list_edit_state.select(Some(0));
        return rs;
    }
}
