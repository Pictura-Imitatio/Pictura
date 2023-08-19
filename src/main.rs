mod image;
mod gui;
use std::env;

fn parse(args: Vec<String>){
    println!("{:?}", args);
    let HELP = "RTFM";
    let VERSION = "0.1.0";
    
    if args.len() == 0 {gui::run()}
    else{
        let temp = "temp";
        
        // TODO: replace outer iter and move it into inner iters
        match &args[0][..] {
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
                    args.iter().for_each(|x|{
                        match &**x {
                            "-o"  =>  println!("Output to file {}", temp ),
                            "-v"  =>  println!("Verbose"),
                            "-cp" =>  println!("Copy to clipboard"),
                            "-t"  =>  println!("Wait {} seconds", temp ),
                                _     =>  println!("Flag not found")
                        }
                    });
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
                        "-o"  =>  println!("Output to file {}", temp),
                        "-cp" =>  println!("Copy to clipboard"),
                        "-t"  =>  println!("Wait {} seconds", temp),
                        _     =>  println!("Flag not found")
                    }
                }
            },

            // all the other cases
            _ => println!("pictura: invalid mode {}",args[0]),
            
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    parse(args);
}
