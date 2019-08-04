use postit::commands;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Must provide a command (e.g build)");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "build" => commands::build("./src", "./dist"),
        _ => println!("Unknown command"),
    }
}
