use crate::ark::{ArkServer, ArkServerMod, Error};
use crate::DB_PATH;

use std::fs;
use tui::widgets::ListState;

pub fn read_db() -> Result<Vec<ArkServer>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn add_ark_server_to_db() -> Result<Vec<ArkServer>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
    parsed.push(ArkServer::named("New Server"));
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn remove_ark_server_at_index(ark_server_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = ark_server_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        ark_server_list_state.select(Some(selected - 1));
        return Ok(())
    }
    return Err(Error::SelectionError)
}

pub fn add_ark_server_mod_to_db(ark_server_list_state: &ListState) -> Result<Vec<ArkServer>, Error> {
    if let Some(selected_server) = ark_server_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
        parsed[selected_server].mods.push(ArkServerMod::named("New Mod"));
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        return Ok(parsed)
    }
    return Err(Error::SelectionError)
}

pub fn remove_ark_server_mod_at_index(ark_server_list_state: &mut ListState, ark_server_mod_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected_server) = ark_server_list_state.selected() {
        if let Some(selected_mod) = ark_server_mod_list_state.selected() {
            let db_content = fs::read_to_string(DB_PATH)?;
            let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
            parsed[selected_server].mods.remove(selected_mod);
            fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
            if selected_mod > 0 {
                ark_server_mod_list_state.select(Some(selected_mod - 1));
            }
            return Ok(())
        }
    }
    return Err(Error::SelectionError)
}
