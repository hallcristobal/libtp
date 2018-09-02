#![no_std]
#![feature(panic_handler)]

extern crate arrayvec;
#[macro_use(report)]
extern crate gcn;

pub mod addrs;
pub mod game;
pub mod lang_items;
pub mod link;
pub mod system;

pub type Addr = usize;
