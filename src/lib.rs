#![feature(unsafe_destructor, net, io)]

extern crate bincode;
extern crate "rustc-serialize" as serialize;
extern crate bchannel;

pub use tcp::{OutTcpStream, InTcpStream, upgrade_tcp, connect_tcp, listen_tcp};
pub use bincode::SizeLimit;

pub mod tcp;
