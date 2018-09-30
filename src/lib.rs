#![no_std]
#![feature(min_const_fn)]

extern crate arrayvec;
#[macro_use(report)]
extern crate gcn;

pub mod addrs;
pub mod game;
pub mod lang_items;
pub mod link;
pub mod system;
pub mod warping;

pub type Addr = usize;
