use std::env;

pub mod yeet;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if let Some(config) = yeet::parse_args(&args) {
        match config.command {
            yeet::Options::Init => {
                yeet::init_repo();
            }
        }
    }
}
