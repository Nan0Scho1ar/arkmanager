use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
    #[error("error, invalid selection")]
    SelectionError,
}

pub enum Event<I> {
    Input(I),
    Tick,
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
