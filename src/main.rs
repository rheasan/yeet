use std::env;

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
                        yeet::hash_file(&path);
                    }
                    None => {
                        unreachable!();
                    }
                }
            }
        }
    }
}
