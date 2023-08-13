use std::env;
fn help(){
    // TODO: modify the help printout 
    println!("help")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1][..];
    // TODO: specify flags tuple for easy extraction
    let flags = &args;
    match mode { 
        // modes
        "--text" => {
            println!("AI text extraction mode enabled");
           // TODO: add AI functionality  
        },
        "--image" => { 
            println!("Image mode enabled");
            // TODO: add screenshot functionality
        },
        "--help" => help(),
        // all the other cases
        _ => {
            // show a help message
            println!("pictura: invalid mode {}",mode);
        }
    }
}
