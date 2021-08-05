use std::io::Write;
// NOTE: enables creation_flags on the command builder, only works on windows
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;

use crate::environment::{client_path, current_time_millis, log_file};
use crate::structs::Message;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

#[cfg(target_os = "windows")]
pub fn send_mail(msg: &Message) -> () {
    let exe = client_path().unwrap();
    if let Err(e) = msg.ensure_attachments() {
        log_to_file("send_mail", &format!("could not ensure attachment: {}", e));
        return;
    }
    Command::new(&exe)
        .args(&[msg.make_mailto_link()])
        .creation_flags(DETACHED_PROCESS)
        .spawn()
        .unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn send_mail(_msg: &Message) -> () {}

pub fn log_to_file(caller: &str, stuff: &str) -> () {
    let written = if let Ok(mut lf) = log_file() {
        writeln!(lf, "{} | {}: {}", current_time_millis(), caller, stuff)
    } else {
        eprintln!("Couldn't open file");
        Ok(())
    };
    if let Err(_) = written {
        eprintln!("Couldn't write to file");
    }
}


