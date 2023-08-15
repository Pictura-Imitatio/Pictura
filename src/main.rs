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
//    println!("{:?}", args);
/*    let args: Vec<String> = env::args().collect();
    let mode = &args[1][..];
    match args.len() {
        1 => {
            gui::run();
        }

        _ => match mode { 

            // modes
            "--gui" => {
                println!("AI text extraction mode enabled");
               // DONE: add call for gui  
               gui::run();
            },

            "--text" => {
                println!("AI text extraction mode enabled");
               // TODO: add AI functionality  
                if args.len()>2{
                    let cmd = &args[2];
                    match &cmd[..] {
                        "-o"  =>  println!("Output to file {}", &args[3][..]),
                        "-v"  =>  println!("Verbose"),
                        "-cp" =>  println!("Copy to clipboard"),
                        "-t"  =>  println!("Wait {} seconds", &args[3][..]),
                        _     =>  println!("Flag not found")
                    }
                }

            },

            "--image" => { 
                println!("Image mode enabled");
                // DONE: add screenshot functionality
                image::run();
                
                if args.len()>2{
                    let cmd = &args[2];
                    match &cmd[..] {
                        "-o"  =>  println!("Output to file {}", &args[3][..]),
                        "-cp" =>  println!("Copy to clipboard"),
                        "-t"  =>  println!("Wait {} seconds", &args[3][..]),
                        _     =>  println!("Flag not found")
                    }
                }

            },

            "--help" => help(),

            // all the other cases
            _ => {
                // show a help message
                println!("pictura: invalid mode {}",mode);
            }
        }
    } */
}
