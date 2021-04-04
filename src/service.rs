use crate::ark::{Error, ProgState};
use crate::db::{get_server_service_name};
use std::process::Command;

pub fn restart_ark_server(state: &ProgState) -> Result<String, Error> {
    let service_name = get_server_service_name(state).expect("Service name exists");
    let output = Command::new("systemctl")
                          .arg("restart_ark_server")
                          .arg(service_name.to_string())
                          .output()
                          .expect("command failed to start");


    let cow = String::from_utf8_lossy(&output.stdout);
    let mut s = cow.to_string();
    if s == "" {
        let cow2 = String::from_utf8_lossy(&output.stderr);
        s = cow2.to_string();
    }
    Ok(s)
}

pub fn start_ark_server(state: &ProgState) -> Result<String, Error> {
    let service_name = get_server_service_name(state).expect("Service name exists");
    let output = Command::new("systemctl")
                          .arg("start")
                          .arg(service_name.to_string())
                          .output()
                          .expect("command failed to start");
    let cow = String::from_utf8_lossy(&output.stdout);
    let mut s = cow.to_string();
    if s == "" {
        let cow2 = String::from_utf8_lossy(&output.stderr);
        s = cow2.to_string();
    }
    Ok(s)
}


pub fn stop_ark_server(state: &ProgState) -> Result<String, Error> {
    let service_name = get_server_service_name(state).expect("Service name exists");
    let output = Command::new("systemctl")
                          .arg("stop")
                          .arg(service_name.to_string())
                          .output()
                          .expect("command failed to start");


    let cow = String::from_utf8_lossy(&output.stdout);
    let mut s = cow.to_string();
    if s == "" {
        let cow2 = String::from_utf8_lossy(&output.stderr);
        s = cow2.to_string();
    }
    Ok(s)
}

pub fn status_ark_server(state: &ProgState) -> Result<String, Error> {
    let service_name = get_server_service_name(state).expect("Service name exists");
    let output = Command::new("systemctl")
                          .arg("is-active")
                          .arg(service_name.to_string())
                          .output()
                          .expect("command failed to start");
    let cow = String::from_utf8_lossy(&output.stdout);
    let mut s = cow.to_string();
    if s == "" {
        let cow2 = String::from_utf8_lossy(&output.stderr);
        s = cow2.to_string();
    }
    Ok(s)
}
