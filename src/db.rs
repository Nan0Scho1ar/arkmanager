use crate::ark::{ArkServer, ArkServerMod, Error, RenderState};
use crate::DB_PATH;

use std::fs;

pub fn read_db() -> Result<Vec<ArkServer>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn get_server(state: &RenderState) -> Result<ArkServer, Error> {
    let ark_server_list = read_db().expect("can fetch ark_server list");
    let selected_ark_server = ark_server_list
        .get(
            state.ark_server_list_state
            .selected()
            .expect("there is always a selected ark_server"),
            )
        .expect("exists")
        .clone();
    return Ok(selected_ark_server);
}

pub fn get_num_servers() -> Result<usize, Error> {
    let ark_server_list = read_db().expect("can fetch ark_server list");
    let num_ark_servers = ark_server_list.len();
    return Ok(num_ark_servers);
}

pub fn get_num_server_mods(state: &RenderState) -> Result<usize, Error> {
    let selected_ark_server = get_server(state).expect("Can get seleted server");
    let num_ark_server_mods = selected_ark_server.mods.len();
    return Ok(num_ark_server_mods);
}

pub fn add_ark_server_to_db() -> Result<Vec<ArkServer>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
    parsed.push(ArkServer::named("New Server"));
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn remove_ark_server_at_index(state: &mut RenderState) -> Result<(), Error> {
    if let Some(selected) = state.ark_server_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        state.ark_server_list_state.select(Some(selected - 1));
        return Ok(())
    }
    return Err(Error::SelectionError)
}

pub fn add_ark_server_mod_to_db(state: &RenderState) -> Result<Vec<ArkServer>, Error> {
    if let Some(selected_server) = state.ark_server_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
        parsed[selected_server].mods.push(ArkServerMod::named("New Mod"));
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        return Ok(parsed)
    }
    return Err(Error::SelectionError)
}

pub fn remove_ark_server_mod_at_index(state: &mut RenderState) -> Result<(), Error> {
    if let Some(selected_server) = state.ark_server_list_state.selected() {
        if let Some(selected_mod) = state.ark_server_mod_list_state.selected() {
            let db_content = fs::read_to_string(DB_PATH)?;
            let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
            parsed[selected_server].mods.remove(selected_mod);
            fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
            if selected_mod > 0 {
                state.ark_server_mod_list_state.select(Some(selected_mod - 1));
            }
            return Ok(())
        }
    }
    return Err(Error::SelectionError)
}
