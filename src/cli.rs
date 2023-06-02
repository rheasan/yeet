pub enum Options {
    Init,
    CatFile,
    HashFile,
    WriteTree,
    ReadTree,
    SetAuthor,
    Commit,
    Log,
    Checkout,
    Tag,
}

pub struct Config {
    pub command: Options,
    pub args: Option<Vec<String>>,
}
pub fn parse_args(args: &Vec<String>) -> Option<Config> {
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
                args: Some(args.get(2..).unwrap().to_vec()),
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
                args: Some(args.get(2..).unwrap().to_vec()),
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
    } else if args[1] == "readtree" {
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
                command: Options::ReadTree,
                args: Some(args.get(2..).unwrap().to_vec()),
            });
        }
    } else if args[1] == "setauthor" {
        if args.len() > 3 {
            println!("Too many arguments");
            print_help();
            return None;
        } else if args.len() != 3 {
            println!("No name provided");
            print_help();
            return None;
        } else {
            return Some(Config {
                command: Options::SetAuthor,
                args: Some(args.get(2..).unwrap().to_vec()),
            });
        }
    } else if args[1] == "commit" {
        if args.len() > 3 {
            println!("Too many arguments");
            print_help();
            return None;
        } else if args.len() != 3 {
            println!("No commit message provided");
            print_help();
            return None;
        } else {
            return Some(Config {
                command: Options::Commit,
                args: Some(args.get(2..).unwrap().to_vec()),
            });
        }
    } else if args[1] == "log" {
        if args.len() > 3 {
            println!("Too many arguments");
            print_help();
            return None;
        } else if args.len() == 2 {
            return Some(Config {
                command: Options::Log,
                args: None,
            });
        } else if args.len() == 3 {
            return Some(Config {
                command: Options::Log,
                args: Some(vec![args[2].clone()]),
            });
        }
    } else if args[1] == "checkout" {
        if args.len() > 3 {
            println!("Too many arguments");
            print_help();
            return None;
        } else if args.len() != 3 {
            println!("No commit id provided");
            print_help();
            return None;
        } else {
            return Some(Config {
                command: Options::Checkout,
                args: Some(args.get(2..).unwrap().to_vec()),
            });
        }
    } else if args[1] == "tag" {
        if args.len() > 4 {
            println!("Too many arguments");
            print_help();
            return None;
        } else if args.len() != 4 {
            println!("No commit id provided");
            print_help();
            return None;
        } else {
            return Some(Config {
                command: Options::Tag,
                args: Some(args.get(2..).unwrap().to_vec()),
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
