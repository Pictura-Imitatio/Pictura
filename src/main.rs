use clap::Parser;
mod image;
mod gui;
mod args;
use args::Argumemts;


fn help(){
    // TODO: modify the help printout 
    println!("help")
}

fn main() {
    let args = Argumemts::parse();
    println!("{:?}", args);
}
