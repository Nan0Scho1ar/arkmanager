use crate::ark::{ArkServer, ArkServerMod, Error, ProgState};
use crate::DB_PATH;

use std::fs;

pub fn get_servers() -> Result<Vec<ArkServer>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn get_server(state: &ProgState) -> Result<ArkServer, Error> {
    let ark_server_list = get_servers().expect("can fetch ark_server list");
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
    let ark_server_list = get_servers().expect("can fetch ark_server list");
    let num_ark_servers = ark_server_list.len();
    return Ok(num_ark_servers);
}

pub fn get_num_server_mods(state: &ProgState) -> Result<usize, Error> {
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

pub fn remove_ark_server_at_index(state: &mut ProgState) -> Result<(), Error> {
    if let Some(selected) = state.ark_server_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        if selected > 0 {
            state.ark_server_list_state.select(Some(selected - 1));
        }
        return Ok(())
    }
    return Err(Error::SelectionError)
}


pub fn set_server_property(state: &mut ProgState) -> Result<(), Error> {
    if let Some(selected_server) = state.ark_server_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
        match state.ark_server_list_edit_state.selected().unwrap() {
            0 => parsed[selected_server].id = state.tmp_server_field.parse::<usize>().unwrap(),
            1 => parsed[selected_server].name = state.tmp_server_field.clone(),
            2 => parsed[selected_server].category = state.tmp_server_field.clone(),
            3 => parsed[selected_server].age = state.tmp_server_field.parse::<usize>().unwrap(),
            4 => parsed[selected_server].service_name = state.tmp_server_field.clone(),
            _ => {}
        }
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        if selected_server > 0 {
            state.ark_server_list_state.select(Some(selected_server - 1));
        }
        return Ok(())
    }
    return Err(Error::SelectionError)
}


pub fn set_server_mod_property(state: &mut ProgState) -> Result<(), Error> {
    if let Some(selected_server) = state.ark_server_list_state.selected() {
        if let Some(selected_mod) = state.ark_server_mod_list_state.selected() {
            let db_content = fs::read_to_string(DB_PATH)?;
            let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
            match state.ark_server_mod_list_edit_state.selected().unwrap() {
                0 => parsed[selected_server].mods[selected_mod].id = state.tmp_mod_field.parse::<usize>().unwrap(),
                1 => parsed[selected_server].mods[selected_mod].name = state.tmp_mod_field.clone(),
                2 => parsed[selected_server].mods[selected_mod].category = state.tmp_mod_field.clone(),
                3 => parsed[selected_server].mods[selected_mod].age = state.tmp_mod_field.parse::<usize>().unwrap(),
                _ => {}
            }
            fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
            if selected_mod > 0 {
                state.ark_server_mod_list_state.select(Some(selected_mod - 1));
            }
            return Ok(())
        }
    }
    return Err(Error::SelectionError)
}



pub fn get_server_mods(state: &ProgState) -> Result<Vec<ArkServerMod>, Error> {
    let selected_ark_server = get_server(state).expect("server exists");
    return Ok(selected_ark_server.mods)
}

pub fn get_server_mods_str(state: &ProgState) -> Result<String, Error> {
    let selected_ark_server_mods = get_server_mods(state).expect("Mods exist");
    let mods_str = selected_ark_server_mods.into_iter().map(|i| i.name + &", ".to_string()).collect::<String>();
    return Ok(mods_str)
}

pub fn get_server_mod(state: &ProgState) -> Result<ArkServerMod, Error> {
    let selected_ark_server_mod = get_server_mods(state)
        .unwrap()
        .get(
            state.ark_server_mod_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();
    return Ok(selected_ark_server_mod)
}

pub fn get_server_properties(state: &ProgState) -> Result<Vec<String>, Error> {
    let selected_ark_server = get_server(state).expect("Server has mod");
    let props = vec![
        selected_ark_server.id.to_string(),
        selected_ark_server.name.to_string(),
        selected_ark_server.category.to_string(),
        selected_ark_server.age.to_string(),
        selected_ark_server.service_name.to_string(),
        selected_ark_server.created_at.to_string(),
    ];
    Ok(props)
}

pub fn get_server_mod_properties(state: &ProgState) -> Result<Vec<String>, Error> {
    let selected_ark_server_mod = get_server_mod(state).expect("Server has mod");
    let props = vec![
        selected_ark_server_mod.id.to_string(),
        selected_ark_server_mod.name.to_string(),
        selected_ark_server_mod.category.to_string(),
        selected_ark_server_mod.age.to_string(),
        selected_ark_server_mod.created_at.to_string(),
    ];
    Ok(props)
}

pub fn get_server_mod_property(state: &ProgState) -> Result<String, Error> {
    let sel = state.ark_server_mod_list_edit_state
        .selected()
        .unwrap();
    let props = get_server_mod_properties(state).expect("Server mod has properties");
    let selected_ark_server_mod_property = props[sel].clone();
    Ok(selected_ark_server_mod_property)
}

pub fn get_server_property(state: &ProgState) -> Result<String, Error> {
    let sel = state.ark_server_list_edit_state
        .selected()
        .unwrap();
    let props = get_server_properties(state).expect("Server has properties");
    let selected_ark_server_property = props[sel].clone();
    Ok(selected_ark_server_property)
}





pub fn add_ark_server_mod_to_db(state: &ProgState) -> Result<Vec<ArkServer>, Error> {
    if let Some(selected_server) = state.ark_server_list_state.selected() {
        let db_content = fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<ArkServer> = serde_json::from_str(&db_content)?;
        parsed[selected_server].mods.push(ArkServerMod::named("New Mod"));
        fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        return Ok(parsed)
    }
    return Err(Error::SelectionError)
}

pub fn remove_ark_server_mod_at_index(state: &mut ProgState) -> Result<(), Error> {
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
