mod image;
mod gui;
use std::{ env, fs };
use screenshots::Screen;

fn parse(args: Vec<String>){
    println!("{:?}", args);
    let help = "RTFM";
    let version = env!("CARGO_PKG_VERSION");
    
    if args.len() == 0 {gui::run()}
    else{
        // DONE: replace outer iter and move it into inner iters
        let mut j = 0;
        while j < args.len(){
            match &args[j][..] {
                "--version"  | "-v" => println!("{}", version),
                "--help"     | "-h" => println!("{}", help),

                // optional gui flag jsut for ocd ppl
                "--gui"             => {
                    println!("AI text extraction mode enabled");
                    // DONE: add call for gui  
                    gui::run()
                },

                // text extraction mode
                "--text"     | "-T" => {
                    println!("AI text extraction mode enabled");
                    // TODO: add AI functionality  
                    if args.len()>1{
                        let mut i = j+1;
                        while i < args.len(){
                            match &args[i][..] {
                                "-o"  =>  {
                                    println!("Output to file {}", &args[i+1][..]);
                                    i = i+1;
                                }
                                "-v"  =>  println!("Verbose"),
                                "-cp" =>  println!("Copy to clipboard"),
                                "-t"  => { 
                                    println!("Wait {} seconds", &args[i+1][..]);
                                    i = i+1;
                                }
                                _     =>  {
                                    j = i-1;
                                    break;
                                }
                            }
                            i = i+1;
                        }
                    }
                },

                        // image / normal sc flag
                "--image"    | "-I" => { 
                    println!("Image mode enabled");
                    // DONE: add screenshot functionality

                    if args.len()>1{
                        let mut i = j+1;
                        while i < args.len(){
                            match &args[i][..] {
                                "--output"    | "-o"  => {
                                    let points;
                                    // TODO: harden this
                                    if i+2 < args.len() {
                                        points = (Some(image::Point{x: args[i+1][..].parse::<i32>().unwrap(), 
                                                                    y: args[i+2][..].parse::<i32>().unwrap()}), 
                                                  Some(image::Point{x: args[i+3][..].parse::<i32>().unwrap(), 
                                                                    y: args[i+4][..].parse::<i32>().unwrap()}));
                                    }
                                    else { 
                                        points = (None, None);
                                    }
                                    let compressed_images = image::run(None, points);
                                    let mut k = 0;
                                    for image in compressed_images {
                                        // TODO: make option and unwrap or for default file location
                                        fs::write(format!("target/{}.png", k), image).unwrap();
                                        k = k + 1;
                                    }
                                    i = i+1;
                                }
                                "--clipboard" | "-cp" =>  println!("Copy to clipboard"),
                                "-t"                  => { 
                                    println!("Wait {} seconds", &args[i+1][..]);
                                    i = i+1;
                                },
                                _                     =>  {
                                    j = i-1;
                                    break;
                                }
                            }
                            i = i+1;
                        }
                    }
                },
                "--display-info"    => {
                    let screens = Screen::all().unwrap();
                    for screen in screens {
                        println!("{screen:?}");
                    }
                },

                // all the other cases
                _ => {
                    if j != args.len()-1{
                        println!("pictura: invalid mode {}", args[j])
                    }else{
                        println!("Executed successfuly!");
                    }
                }
                
            }
            j = j+1;
        }
        
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    parse(args);
}
