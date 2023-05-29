use std::{env, fs, path::PathBuf};

use crate::data;

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

pub fn hash_file(path: PathBuf) {
    let res = data::hash_file(&path);
    match res {
        Ok(file_data) => {
            println!(
                "{} {} {}",
                file_data.file_type, file_data.hash, file_data.file_name
            );
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn write_tree(path: PathBuf) {
    let dir_entries = fs::read_dir(path.to_owned()).expect("Failed to read directory");

    let ignore_path = path.to_owned().join(".yeetignore");
    let mut ignore_entries: Vec<String> = vec![String::from(".yeet")];

    if fs::try_exists(ignore_path.to_owned()).expect("cant read files") {
        let mut other_entries = fs::read_to_string(ignore_path.to_owned())
            .unwrap()
            .split('\n')
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        ignore_entries.append(&mut other_entries);
    }

    for i in dir_entries {
        let entry = i.expect("Failed to read entry");
        let filename = entry.file_name().to_str().unwrap().to_string();

        if ignore_entries.contains(&filename) {
            continue;
        }

        let file_metadata = entry.metadata().expect("Failed to read metadata");

        if file_metadata.is_dir() {
            write_tree(entry.path());
        } else {
            println!("{}", filename);
        }
    }
}
