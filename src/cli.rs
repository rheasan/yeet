pub enum Options {
    Init,
    CatFile,
    HashFile,
    WriteTree,
}

pub struct Config {
    pub command: Options,
    pub args: Option<String>,
}
pub fn parse_args(args: &[String]) -> Option<Config> {
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
                command: Options::Init,
                args: None,
            });
        }
    } else if args[1] == "catfile" {
        if args.len() > 3 {
            println!("Too many arguments");
            print_help();
            return None;
        } else if args.len() != 3 {
            println!("No file name provided");
            print_help();
            return None;
        } else {
            return Some(Config {
                command: Options::CatFile,
                args: Some(args[2].clone()),
            });
        }
    } else if args[1] == "hashfile" {
        if args.len() > 3 {
            println!("Too many arguments");
            print_help();
            return None;
        } else if args.len() != 3 {
            println!("No file name provided");
            print_help();
            return None;
        } else {
            return Some(Config {
                command: Options::HashFile,
                args: Some(args[2].clone()),
            });
        }
    } else if args[1] == "writetree" {
        if args.len() > 2 {
            println!("Too many arguments");
            print_help();
            return None;
        } else {
            return Some(Config {
                command: Options::WriteTree,
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