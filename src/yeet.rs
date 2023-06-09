use std::{env, fs, io::Write, path::PathBuf, process::exit};

use time::OffsetDateTime;

use crate::data::{self, hash_dir, read_commit, write_obj_hash, FileData};

pub fn init_repo() {
    const INITIAL_HEAD: &[u8] = "initial".as_bytes();
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

    fs::create_dir("./.yeet/objects").expect("Error creating objects");

    fs::create_dir("./.yeet/repo_data").expect("Error creating repo_data");

    fs::create_dir_all("./.yeet/refs/tags").expect("Error creating tags");

    let mut head = fs::File::create("./.yeet/refs/tags/HEAD").expect("Error setting head");

    head.write(INITIAL_HEAD).expect("Error setting head");
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

pub fn hash_file(path: PathBuf, show_out: bool) -> Result<u64, std::io::Error> {
    let file_data = fs::read(&path)?;
    // println!("{:?}", file_data);
    let hash = write_obj_hash(&file_data, "blob".to_string())?;

    if show_out {
        println!("blob {} {:?}", hash, path.file_name().unwrap());
    }
    return Ok(hash);
}

pub fn write_tree(path: PathBuf) -> Result<u64, std::io::Error> {
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
            let hash = write_tree(entry.path())?;
            let d = FileData {
                file_name: filename,
                file_type: "tree".to_string(),
                hash,
            };
            cur_dir_data.push(d)
        } else {
            let hash = hash_file(entry.path(), false)?;
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
    let write_dir_name = format!("{:?}", write_dir.as_os_str());
    let root_dir = data::gen_tree(hash, write_dir_name, write_dir.to_owned());

    if let Err(e) = root_dir {
        println!("Error: {}", e.to_string());
        exit(1);
    }

    if fs::try_exists(write_dir.to_owned()).expect("Unable to read dir") {
        fs::remove_dir_all(write_dir.to_owned()).expect("Unable to remove previous revision");
    }

    fs::create_dir_all(write_dir.to_owned()).expect("Unable to make dir");
    data::write_entry(root_dir.unwrap());
}

pub fn set_author(name: String) {
    let mut auth_file =
        fs::File::create(PathBuf::from("./.yeet/repo_data/author")).expect("Unable to find data");
    auth_file
        .write(name.as_bytes())
        .expect("Unable to set auth name: ");
    println!("Set author name to {}", name);
}

pub fn commit(message: String) -> Result<(), std::io::Error> {
    let author = fs::read_to_string(PathBuf::from("./.yeet/repo_data/author"));
    if let Err(_) = author {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Author name not found",
        ));
    }
    let time = OffsetDateTime::now_utc();
    let mut parent = data::get_tag(&"HEAD".to_string()).unwrap();
    if parent == "" {
        parent = "initial".to_string();
    }
    let id = write_tree(PathBuf::from("."))?;
    let commit_data = format!(
        "tree {}\nparent {}\nauthor {}\ntime {:?}\n{}",
        id,
        parent,
        author.unwrap(),
        time,
        message
    );
    let commit_id = data::write_obj_hash(commit_data.as_bytes(), "commit".to_string())?;

    data::set_tag("HEAD".to_string(), commit_id.to_string()).unwrap();
    println!("commit id: {}", commit_id);
    println!("{}", message);

    Ok(())
}

pub fn log(commit_id: String) {
    let res = read_commit(commit_id);
    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }
}

pub fn checkout(commit_id: String) {
    let tree_hash = data::get_commit_tree(&commit_id);
    if let Err(e) = tree_hash {
        eprintln!("Error: {}", e.to_string());
    } else {
        data::set_tag("HEAD".to_string(), commit_id).expect("Failed to set head");
        read_tree(tree_hash.unwrap(), PathBuf::from("./restored"));
    }
}

pub fn tag_commit(tag: String, hash: String) {
    println!("tag {} hash {}", tag, hash);
    if let Ok(_) = tag.parse::<u64>() {
        eprintln!("Error: Cannot use integer as tag name: {}", tag);
        return;
    }
    let res = data::set_tag(tag, hash);
    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }
}

pub fn k() {
    if let Err(e) = data::print_all_refs() {
        eprintln!("Error: {}", e);
    }
}
