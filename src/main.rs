use std::{env, process::exit};

pub mod yeet;

// TODO: clean up 3-depth match (monkaW)
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
                        let res = yeet::cat_file(&path, &String::from("./.yeet/objects"));
                        match res {
                            Ok(data) => {
                                // data = [data_type, file_data]
                                println!(
                                    "obj-type: {}",
                                    String::from_utf8(data[0].clone()).unwrap()
                                );
                                println!("file-data: {:?}", data[1]);
                                if let Ok(file_data) = String::from_utf8(data[1].clone()) {
                                    println!("ascii: {}", file_data);
                                } else {
                                    println!("no ascii")
                                }
                            }
                            Err(e) => {
                                println!("{}", e);
                            }
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
