use std::{env, fs, process::exit};

enum YeetOptions {
    Init,
}

struct Config {
    command: YeetOptions,
    args: Option<Vec<String>>,
}

fn parse_args(args: &[String]) -> Option<Config> {
    // No arguments given
    if args.len() == 1 {
        println!("No args provided");
        print_help();
        return None;
    }

    // Initialize an empty repo in current directory
    // should have no extra args
    if args[1] == "init" {
        if args.len() > 2 {
            println!("Too many arguments");
            print_help();
            return None;
        } else {
            return Some(Config {
                command: YeetOptions::Init,
                args: None,
            });
        }
    }

    println!("Bad arguments");
    print_help();
    return None;
}

fn print_help() {
    println!("Usage: TODO");
}

fn init_repo() {
    let res = fs::create_dir("./.yeet");
    match res {
        Err(e) => {
            println!("Error creating repo: {}", e);
            exit(1);
        }
        Ok(_) => {
            println!(
                "created empty repo in {}",
                env::current_dir().unwrap().to_string_lossy()
            );
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if let Some(config) = parse_args(&args) {
        match config.command {
            YeetOptions::Init => {
                init_repo();
            }
        }
    }
}
