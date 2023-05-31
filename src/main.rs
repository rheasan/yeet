#![feature(fs_try_exists)]
use std::{env, path::PathBuf};

pub mod cli;
pub mod data;
pub mod yeet;

// TODO: clean up 3-depth match (monkaW)
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if let Some(config) = cli::parse_args(&args) {
        match config.command {
            cli::Options::Init => {
                yeet::init_repo();
            }
            cli::Options::CatFile => {
                let file_path = config.args;
                match file_path {
                    Some(path) => {
                        yeet::cat_file(&path);
                    }
                    None => {
                        unreachable!();
                    }
                }
            }
            cli::Options::HashFile => {
                let file_path = config.args;
                match file_path {
                    Some(path) => {
                        yeet::hash_file(PathBuf::from(path), true);
                    }
                    None => {
                        unreachable!();
                    }
                }
            }
            cli::Options::WriteTree => {
                let rev_id = yeet::write_tree(PathBuf::from("."));
                println!("New revision id: {}", rev_id);
            }
            cli::Options::ReadTree => {
                let hash = config.args.unwrap();
                yeet::read_tree(hash, PathBuf::from("./restored"));
            }
        }
    }
}
