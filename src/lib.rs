#![no_std]
#![feature(panic_implementation)]

extern crate arrayvec;
extern crate gcn;

pub mod addrs;
pub mod game;
pub mod lang_items;
pub mod system;
pub mod link;

pub type Addr = usize;
