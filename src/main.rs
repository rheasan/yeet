use std::{env, process::exit};

pub mod yeet;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if let Some(config) = yeet::parse_args(&args) {
        match config.command {
            yeet::Options::Init => {
                if let Err(e) = yeet::init_repo() {
                    println!("{}", e);
                    exit(1);
                }
            }
            yeet::Options::CatFile => {
                let file_path = config.args;
                match file_path {
                    Some(path) => {
                        if let Err(e) = yeet::cat_file(&path) {
                            println!("Error: {}", e);
                        }
                    }
                    None => {
                        unreachable!();
                    }
                }
            }
            yeet::Options::HashFile => {
                let file_path = config.args;
                match file_path {
                    Some(path) => {
                        if let Err(e) = yeet::hash_file(&path) {
                            println!("Error: {}", e);
                        }
                    }
                    None => {
                        unreachable!();
                    }
                }
            }
        }
    }
}
