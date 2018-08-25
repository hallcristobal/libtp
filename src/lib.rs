#![no_std]
#![feature(panic_implementation)]

extern crate arrayvec;
extern crate gcn;

pub mod addrs;
pub mod game;
pub mod lang_items;
pub mod link;
pub mod system;

pub type Addr = usize;
