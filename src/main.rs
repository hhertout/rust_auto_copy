use std::env;
use std::mem;
use std::process;

use initializer::Initializer;

pub mod logger;

use logger::Logger;

fn main() {
    let args: Vec<String> = env::args().collect();
    let main_arg = args[1].clone();
    mem::drop(args);

    match main_arg.as_str() {
        "init" => {
            let filename = String::from("config.yml");
            let initializer = Initializer::new(filename.clone());
            Logger::init_start();
            match initializer.create_yml(None) {
                Ok(_) => Logger::init_success(&filename, None),
                Err(err) => println!("{}", err),
            };
        }
        _ => {
            println!("Error : You provide a wrong arg");
            process::exit(1);
        }
    }
}
