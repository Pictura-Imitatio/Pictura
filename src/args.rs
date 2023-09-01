use screenshots::{self, Screen, DisplayInfo};
use winit::dpi::PhysicalPosition;
use std::fs;
mod image_proc;
use image;
use crate::gui::{self, App};

pub fn parse(args: Vec<String>) -> () {
    let mut image: image::RgbaImage;
    println!("{:?}", args);
    let help = "RTFM";
    let version = env!("CARGO_PKG_VERSION");
    
    if args.len() == 0 {()}
    // DONE: replace outer iter and move it into inner iters
    let mut j = 0;
    while j < args.len(){
        match &args[j][..] {
            "--version"  | "-v" => println!("{}", version),
            "--help"     | "-h" => println!("{}", help),

            // optional gui flag jsut for ocd ppl
            "--gui"             => {
                println!("gui mode");
                let screens = Screen::all().unwrap();
                let mut pos = PhysicalPosition::new(0.0, 0.0);
                let mut br  = PhysicalPosition::new(0.0, 0.0);
                for screen in screens {
                    let screen_pos = PhysicalPosition::new(screen.display_info.x as f64, screen.display_info.y as f64);
                    let screen_br  = PhysicalPosition::new(screen_pos.x as f64 + screen.display_info.width as f64,
                                                          screen_pos.y as f64 + screen.display_info.height as f64);
                    if pos.x > screen_pos.x {
                        pos.x = screen_pos.x;
                    }
                    if pos.y > screen_pos.y {
                        pos.y = screen_pos.y;
                    }
                    if br.x < screen_br.x {
                        br.x = screen_br.x;
                    }
                    if br.y < screen_br.y {
                        br.y = screen_br.y;
                    }

                }
                println!("{:?}\n{:?}", pos, br);
                let app = gui::run(pos, br);
                ()
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
                            "-v"  =>  {
                                println!("Verbose");
                            },
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
                                    points = (Some(image_proc::Point{x: args[i+1][..].parse::<i32>().unwrap(), 
                                        y: args[i+2][..].parse::<i32>().unwrap()}), 
                                              Some(image_proc::Point{x: args[i+3][..].parse::<i32>().unwrap(), 
                                                  y: args[i+4][..].parse::<i32>().unwrap()}));
                                }
                                else { 
                                    points = (None, None);
                                }
                                let compressed_images = image_proc::run(None, points);
                                let mut k = 0;
                                for images in compressed_images {
                                    // TODO: make option and unwrap or for default file location
                                    fs::write(format!("target/{}.png", k), images).unwrap();
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

pub fn capture(app: (PhysicalPosition<f64>, PhysicalPosition<f64>)) {
    let points = (Some(image_proc::Point{x: app.0.x as i32, y: app.0.y as i32 }),
                  Some(image_proc::Point{x: app.1.x as i32, y: app.1.y as i32 }));
    let compressed_images = image_proc::run(None, points);
    let mut k = 0;
    for images in compressed_images {
        fs::write(format!("target/{}.png", k), images).unwrap();
        k = k + 1;
    }
}
