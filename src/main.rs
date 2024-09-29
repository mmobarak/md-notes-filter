use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Please provide a file name as an argument.");
        return;
    }

    let file = &args[1];
    println!("File argument provided: {}", file);
}
