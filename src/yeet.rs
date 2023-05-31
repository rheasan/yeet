use std::{env, fs, path::PathBuf};

use crate::data::{self, hash_dir, write_obj_hash, DirType, FileData};

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
                println!("ascii:\n{}", file_data);
            } else {
                println!("no ascii")
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn hash_file(path: PathBuf, show_out: bool) -> u64 {
    let file_data = fs::read(&path).expect("Error reading file");
    // println!("{:?}", file_data);
    let hash = write_obj_hash(&file_data, "blob".to_string());

    if show_out {
        println!("blob {} {:?}", hash, path.file_name().unwrap());
    }
    return hash;
}

pub fn write_tree(path: PathBuf) -> u64 {
    let dir_entries = fs::read_dir(path.to_owned()).expect("Failed to read directory");
    let mut cur_dir_data: Vec<FileData> = vec![];

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
            let hash = write_tree(entry.path());
            let d = FileData {
                file_name: filename,
                file_type: "tree".to_string(),
                hash,
            };
            cur_dir_data.push(d);
        } else {
            let hash = hash_file(entry.path(), false);
            let d = FileData {
                file_name: filename,
                file_type: "blob".to_string(),
                hash,
            };
            cur_dir_data.push(d);
        }
    }

    return hash_dir(&cur_dir_data);
}

pub fn read_tree(hash: String, write_dir: PathBuf) {
    if !fs::try_exists(write_dir.to_owned()).expect("Unable to read dir") {
        fs::remove_dir_all(write_dir.to_owned()).expect("Unable to remove previous revision");
    }

    fs::create_dir_all(write_dir.to_owned()).expect("Unable to make dir");

    let write_dir_name = format!("{:?}", write_dir.as_os_str());

    let root_dir = data::gen_tree(hash, write_dir_name, write_dir);
    data::write_entry(root_dir);
}
