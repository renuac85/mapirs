#[macro_use]
extern crate bitflags;
extern crate directories;
extern crate urlencoding;
extern crate winreg;

pub use crate::ffi::{
    MAPIAddress, MAPIDeleteMail, MAPIDetails, MAPIFindNext, MAPIFreeBuffer, MAPILogoff, MAPILogon,
    MAPIReadMail, MAPIResolveName, MAPISaveMail, MAPISendDocuments, MAPISendMail,
};

// type aliases to centrally define C <-> Rust type conversions
mod types;
// the main data structures from MAPI.h and their safe counterparts
mod structs;
// flag & enum definitions from MAPI.h
mod flags;
// responsible for formatting the commands to the client
mod commands;
// responsible for finding out where the client is installed
mod environment;
// the external API surface exposed to windows
mod ffi;
// path with a file_name() method that's guaranteed to return a value
mod file_path;
