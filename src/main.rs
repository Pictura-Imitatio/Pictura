mod image;
mod gui;
use std::env;

fn parse(args: Vec<String>){
    println!("{:?}", args);
    let HELP = "RTFM";
    let VERSION = "0.1.0";
    
    if args.len() == 0 {gui::run()}
    else{
    args.iter().for_each(|x|{
        // TODO: replace outer iter and move it into inner iters
        match &**x {
            "--version" => println!("{}", VERSION),
            "--help" => println!("{}", HELP),
            "-v" => println!("{}", VERSION),
            "-h" => println!("{}", HELP),

            // optional gui flag jsut for ocd ppl
            "--gui" => {
                println!("AI text extraction mode enabled");
               // DONE: add call for gui  
               gui::run()
            },

            // text extraction mode
            "--text" => {
                println!("AI text extraction mode enabled");
               // TODO: add AI functionality  
                if args.len()>1{
                    let cmd = &args[1];
                    match &cmd[..] {
                        "-o"  =>  println!("Output to file {}", &args[2][..]),
                        "-v"  =>  println!("Verbose"),
                        "-cp" =>  println!("Copy to clipboard"),
                        "-t"  =>  println!("Wait {} seconds", &args[2][..]),
                        _     =>  println!("Flag not found")
                    }
                }
            },

            // image / normal sc flag
            "--image" => { 
                println!("Image mode enabled");
                // DONE: add screenshot functionality
                image::run();
                
                if args.len()>1{
                    let cmd = &args[1];
                    match &cmd[..] {
                        "-o"  =>  println!("Output to file {}", &args[2][..]),
                        "-cp" =>  println!("Copy to clipboard"),
                        "-t"  =>  println!("Wait {} seconds", &args[2][..]),
                        _     =>  println!("Flag not found")
                    }
                }
            },

            // all the other cases
            _ => println!("pictura: invalid mode {}",args[0]),
        }
    });
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    parse(args);
}
