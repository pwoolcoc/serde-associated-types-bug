#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;

use std::fmt;
use serde::{Serialize};

pub trait MyTrait: fmt::Debug + Serialize {}

#[derive(Debug, Serialize)]
pub struct MyStruct<N, T: MyTrait = u32> {
    one: N,
    two: T,
}

#[test]
fn it_works() {
}
