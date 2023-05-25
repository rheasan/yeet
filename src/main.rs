use std::env;

enum YeetOptions {
    Init,
}

fn parse_args() -> Option<(YeetOptions, Option<Vec<String>>)> {
    let args = env::args().collect::<Vec<String>>();

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
            return Some((YeetOptions::Init, None));
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
    todo!();
}

fn main() {
    let args = parse_args();
    if let Some(a) = args {
        match a.0 {
            YeetOptions::Init => {
                init_repo();
            }
        }
    }
}
