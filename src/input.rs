use crate::ark::{RenderState, ArkServerMod, Error, MenuItem, Event, InputEvent};
use crate::service::{start_stop_ark_server, restart_ark_server};
use crate::db::{get_server, get_num_servers, get_num_server_mods, add_ark_server_mod_to_db, remove_ark_server_mod_at_index, read_db, add_ark_server_to_db, remove_ark_server_at_index};
use tui::{ widgets::{ListState, TableState} };
use crossterm::{ event::{KeyCode} };

//Process user input
pub fn process_input(state: &mut RenderState, input: Event<crossterm::event::KeyEvent>) -> Result<InputEvent, Error> {
    if state.editing_server {
        process_server_edits(state, input).expect("Server edit processed");
    } else if state.editing_mod {
        process_mod_edits(state, input).expect("Mod edit processed");
    } else {
        match input {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    return Ok(InputEvent::Exit);
                }
                KeyCode::Char('h') => state.active_menu_item = MenuItem::Home,
                KeyCode::Char('s') => state.active_menu_item = MenuItem::Servers,
                _ => {
                    match state.active_menu_item {
                        MenuItem::Home => process_home(state, event).expect("Processed home"),
                        MenuItem::ViewServer => process_view_server(state, event).expect("Processed view server"),
                        MenuItem::ViewMod => process_view_mod(state, event).expect("Processed view mod"),
                        MenuItem::EditMod => process_edit_mod(state, event).expect("Processed edit mod"),
                        MenuItem::ServerMods => process_server_mods(state, event).expect("Processed server mods"),
                        MenuItem::Servers => process_servers(state, event).expect("Processed servers"),
                    }
                }
            },
            Event::Tick => {}
        }
    }
    Ok(InputEvent::Other)
}

pub fn process_server_edits(state: &mut RenderState, input: Event<crossterm::event::KeyEvent>) -> Result<InputEvent, Error> {
    match input {
        Event::Input(event) => match event.code {
            KeyCode::Enter => {
                state.editing_server = false;
            }
            KeyCode::Backspace => {
            }
            _ => {
                state.tmp_server_field += get_input_char(event.code);
            }
        },
        Event::Tick => {}
    }
    Ok(InputEvent::Other)
}

pub fn process_mod_edits(state: &mut RenderState, input: Event<crossterm::event::KeyEvent>) -> Result<InputEvent, Error> {
    match input {
        Event::Input(event) => match event.code {
            KeyCode::Enter => {
                state.editing_mod = false;
            }
            KeyCode::Backspace => {
            }
            _ => {
                state.tmp_mod_field += get_input_char(event.code);
            }
        },
        Event::Tick => {}
    }
    Ok(InputEvent::Other)
}

pub fn process_home(state: &mut RenderState, event: crossterm::event::KeyEvent) -> Result<(), Error> {
    match event.code {
        _ => {}
    }
    Ok(())
}

pub fn process_view_server(state: &mut RenderState, event: crossterm::event::KeyEvent) -> Result<(), Error> {
    match event.code {
        KeyCode::Char('m') => {
            state.ark_server_mod_list_state.select(Some(0));
            state.active_menu_item = MenuItem::ServerMods
        }
        KeyCode::Char('b') => {
            state.active_menu_item = MenuItem::Servers
        }
        _ => {}
    }
    Ok(())
}

pub fn process_servers(state: &mut RenderState, event: crossterm::event::KeyEvent) -> Result<(), Error> {
    match event.code {
        KeyCode::Char('a') => {
            add_ark_server_to_db().expect("can add new random ark_server");
        }
        KeyCode::Char('d') => {
            remove_ark_server_at_index(state).expect("can remove ark_server");
        }
        KeyCode::Char('s') => {
            start_stop_ark_server(&state).expect("can start ark_server");
        }
        KeyCode::Char('r') => {
            restart_ark_server(&state).expect("can restart ark_server");
        }
        KeyCode::Enter => {
            state.active_menu_item = MenuItem::ViewServer;
        }
        KeyCode::Down => {
            let num_ark_servers = get_num_servers().expect("can fetch ark_server list length");
            try_change_list_state(KeyCode::Down, &mut state.ark_server_list_state, num_ark_servers);
        }
        KeyCode::Up => {
            let num_ark_servers = get_num_servers().expect("can fetch ark_server list length");
            try_change_list_state(KeyCode::Up, &mut state.ark_server_list_state, num_ark_servers);
        }
        _ => {}
    }
    Ok(())
}

pub fn process_server_mods(state: &mut RenderState, event: crossterm::event::KeyEvent) -> Result<(), Error> {
    match event.code {
        KeyCode::Char('a') => {
            add_ark_server_mod_to_db(&state).expect("can add new random ark_server");
        }
        KeyCode::Char('d') => {
            remove_ark_server_mod_at_index(state).expect("can remove ark_server mod");
        }
        KeyCode::Char('b') => {
            state.active_menu_item = MenuItem::ViewServer
        }
        KeyCode::Enter => {
            state.active_menu_item = MenuItem::ViewMod;
        }
        KeyCode::Down => {
            let num_ark_server_mods = get_num_server_mods(&state).expect("Can get seleted server");
            try_change_list_state(KeyCode::Down, &mut state.ark_server_mod_list_state, num_ark_server_mods);
        }
        KeyCode::Up => {
            let num_ark_server_mods = get_num_server_mods(&state).expect("Can get seleted server");
            try_change_list_state(KeyCode::Up, &mut state.ark_server_mod_list_state, num_ark_server_mods);
        }
        _ => {}
    }
    Ok(())
}

pub fn process_edit_mod(state: &mut RenderState, event: crossterm::event::KeyEvent) -> Result<(), Error> {
    match event.code {
        KeyCode::Char('b') => {
            state.active_menu_item = MenuItem::ViewMod
        }
        KeyCode::Enter => {
            state.editing_mod = true;
        }
        KeyCode::Down => {
            try_change_table_state(KeyCode::Down, &mut state.ark_server_mod_list_edit_state, state.num_ark_server_mod_properties-1);
        }
        KeyCode::Up => {
            try_change_table_state(KeyCode::Up, &mut state.ark_server_mod_list_edit_state, state.num_ark_server_mod_properties-1);
        }
        _ => {}
    }
    Ok(())
}

pub fn process_view_mod(state: &mut RenderState, event: crossterm::event::KeyEvent) -> Result<(), Error> {
    match event.code {
        KeyCode::Char('b') => {
            state.active_menu_item = MenuItem::ServerMods
        }
        KeyCode::Char('t') => {
            //TODO Toggle server mod
        }
        KeyCode::Char('e') => {
            state.active_menu_item = MenuItem::EditMod
        }
        _ => {}
    }
    Ok(())
}

//There has to be a better way to do this but I can't find it.
fn get_input_char(code: KeyCode) -> &'static str {
    match code {
        KeyCode::Char('a') => return "a",
        KeyCode::Char('b') => return "b",
        KeyCode::Char('c') => return "c",
        KeyCode::Char('d') => return "d",
        KeyCode::Char('e') => return "e",
        KeyCode::Char('f') => return "f",
        KeyCode::Char('g') => return "g",
        KeyCode::Char('h') => return "h",
        KeyCode::Char('i') => return "i",
        KeyCode::Char('j') => return "j",
        KeyCode::Char('k') => return "k",
        KeyCode::Char('l') => return "l",
        KeyCode::Char('m') => return "m",
        KeyCode::Char('n') => return "n",
        KeyCode::Char('o') => return "o",
        KeyCode::Char('p') => return "p",
        KeyCode::Char('q') => return "q",
        KeyCode::Char('r') => return "r",
        KeyCode::Char('s') => return "s",
        KeyCode::Char('t') => return "t",
        KeyCode::Char('u') => return "u",
        KeyCode::Char('v') => return "v",
        KeyCode::Char('w') => return "w",
        KeyCode::Char('x') => return "x",
        KeyCode::Char('y') => return "y",
        KeyCode::Char('z') => return "z",
        KeyCode::Char('0') => return "0",
        KeyCode::Char('1') => return "1",
        KeyCode::Char('2') => return "2",
        KeyCode::Char('3') => return "3",
        KeyCode::Char('4') => return "4",
        KeyCode::Char('5') => return "5",
        KeyCode::Char('6') => return "6",
        KeyCode::Char('7') => return "7",
        KeyCode::Char('8') => return "8",
        KeyCode::Char('9') => return "9",
        KeyCode::Char(' ') => return " ",
        _ => return "",
    }
}

//Change the list state to the next element unless there are no elements before or after.
fn try_change_list_state(code: KeyCode, list_state: &mut ListState, list_size: usize) {
    match code {
        KeyCode::Down => {
            if let Some(selected) = list_state.selected() {
                if list_size < 1 {
                    list_state.select(Some(0));
                } else if selected >= list_size - 1 {
                    list_state.select(Some(0));
                } else {
                    list_state.select(Some(selected + 1));
                }
            }
        }
        KeyCode::Up => {
            if let Some(selected) = list_state.selected() {
                if list_size < 1 {
                    list_state.select(Some(0));
                } else if selected > 0 {
                    list_state.select(Some(selected - 1));
                } else {
                    list_state.select(Some(list_size - 1));
                }
            }
        }
        _ => return
    }
}

//Change the table state to the next element unless there are no elements before or after.
fn try_change_table_state(code: KeyCode, table_state: &mut TableState, table_size: usize) {
    match code {
        KeyCode::Down => {
            if let Some(selected) = table_state.selected() {
                if table_size < 1 {
                    table_state.select(Some(0));
                } else if selected >= table_size - 1 {
                    table_state.select(Some(0));
                } else {
                    table_state.select(Some(selected + 1));
                }
            }
        }
        KeyCode::Up => {
            if let Some(selected) = table_state.selected() {
                if table_size < 1 {
                    table_state.select(Some(0));
                } else if selected > 0 {
                    table_state.select(Some(selected - 1));
                } else {
                    table_state.select(Some(table_size - 1));
                }
            }
        }
        _ => return
    }
}
