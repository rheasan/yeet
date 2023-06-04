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
                        yeet::cat_file(&path[0]);
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
                        yeet::hash_file(PathBuf::from(&path[0]), true).unwrap();
                    }
                    None => {
                        unreachable!();
                    }
                }
            }
            cli::Options::WriteTree => {
                let rev_id = yeet::write_tree(PathBuf::from(".")).expect("Error writing tree");
                println!("New revision id: {}", rev_id);
            }
            cli::Options::ReadTree => {
                let hash = config.args.unwrap()[0].clone();
                yeet::read_tree(hash, PathBuf::from("./restored"));
            }
            cli::Options::SetAuthor => {
                let name = config.args.unwrap()[0].clone();
                yeet::set_author(name);
            }
            cli::Options::Commit => {
                let message = config.args.unwrap()[0].clone();
                if let Err(e) = yeet::commit(message) {
                    println!("Error: {}", e);
                }
            }
            cli::Options::Log => {
                let hash = config.args.unwrap()[0].clone();
                yeet::log(hash);
            }
            cli::Options::Checkout => {
                let hash = config.args.unwrap()[0].clone();
                yeet::checkout(hash);
            }
            cli::Options::Tag => {
                let args = config.args.unwrap();
                let tag = args[0].clone();
                let hash = args[1].clone();
                yeet::tag_commit(tag, hash);
            }
        }
    }
}
