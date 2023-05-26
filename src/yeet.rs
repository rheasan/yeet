use crate::data;
use std::{env, fs};

pub fn init_repo() {
    let res = fs::create_dir("./.yeet");
    match res {
        Err(e) => {
            println!("Error creating directory: {}", e.to_string());
            return;
        }
        Ok(_) => {
            println!(
                "created empty repo in {}",
                env::current_dir().unwrap().to_string_lossy()
            );
        }
    }

    let res = fs::create_dir("./.yeet/objects");
    if let Err(e) = res {
        println!("Error creating directory: {}", e.to_string());
    }
}

pub fn cat_file(hash: &String) {
    let res = data::get_data(hash, String::from("./.yeet/objects/"));
    match res {
        Ok(data) => {
            // data = [data_type, file_data]
            println!("obj-type: {}", String::from_utf8(data[0].clone()).unwrap());
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

pub fn hash_file(path: &String) {
    let res = data::save_hash(path);
    if let Err(e) = res {
        println!("Error: {}", e);
    }
}

pub fn write_tree(path: String) {
    let dir_entries = fs::read_dir(path).expect("Failed to read directory");

    for i in dir_entries {
        let entry = i.expect("Failed to read entry");
        if entry.file_name() == ".git" || entry.file_name() == "target" {
            continue;
        }
        let file_metadata = entry.metadata().expect("Failed to read metadata");

        if file_metadata.is_dir() {
            let path = entry.path().clone();
            write_tree(String::from(path.to_str().unwrap()));
        } else {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }
}
