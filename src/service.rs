use crate::ark::{ArkServer, Error, ProgState};
use crate::DB_PATH;

use std::fs;
use tui::widgets::ListState;
use std::process::Command;
use std::io::{self, Write};

pub fn restart_ark_server(state: &ProgState) -> Result<(), Error> {
    let output = Command::new("systemctl")
                          .arg("restart_ark_server")
                          .arg("arkserver")
                          .output()
                          .expect("command failed to start");


    eprintln!("Server status {}", output.status);
    return Err(Error::SelectionError)
}

pub fn start_stop_ark_server(state: &ProgState) -> Result<(), Error> {
    let output = Command::new("systemctl")
                          .arg("status")
                          .arg("arkserver")
                          .output()
                          .expect("command failed to start");


    eprintln!("Server status {}", output.status);
    return Err(Error::SelectionError)
}
