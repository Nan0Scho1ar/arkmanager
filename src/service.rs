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

pub fn start_ark_server(state: &ProgState) -> Result<(), Error> {
    let output = Command::new("systemctl")
                          .arg("start")
                          .arg("arkserver")
                          .output()
                          .expect("command failed to start");
    eprintln!("Server status {}", output.status);
    return Err(Error::SelectionError)
}


pub fn stop_ark_server(state: &ProgState) -> Result<(), Error> {
    let output = Command::new("systemctl")
                          .arg("stop")
                          .arg("arkserver")
                          .output()
                          .expect("command failed to start");


    eprintln!("Server status {}", output.status);
    return Err(Error::SelectionError)
}

pub fn status_ark_server(state: &ProgState) -> Result<String, Error> {
    let output = Command::new("systemctl")
                          .arg("is-active")
                          .arg("sshd")
                          .output()
                          .expect("command failed to start");
    let cow = String::from_utf8_lossy(&output.stdout);
    let mut s = cow.to_string();
    if s == "" {
        let cow2 = String::from_utf8_lossy(&output.stderr);
    }
    s = cow.to_string();
    Ok(s)
}
