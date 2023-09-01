#![allow(non_snake_case)]

mod args;
mod gui;
use std::env;

fn main() {
    simple_logger::init_with_env().unwrap();

    let args: Vec<String> = env::args().skip(1).collect();
    args::parse(args);
}
