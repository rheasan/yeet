use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::{env, fs};

pub enum Options {
    Init,
    CatFile,
    HashFile,
}

pub struct Config {
    pub command: Options,
    pub args: Option<String>,
}

const SEPARATOR: u8 = 0x00u8;

pub fn init_repo() -> Result<(), String> {
    let res = fs::create_dir("./.yeet");
    match res {
        Err(e) => {
            return Err(String::from("Error creating directory: ") + e.to_string().as_str());
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
        return Err(String::from("Error creating directory: ") + e.to_string().as_str());
    }

    Ok(())
}

pub fn parse_args(args: &[String]) -> Option<Config> {
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
                args: Some(args[2].clone()),
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
                args: Some(args[2].clone()),
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

fn hash_obj(t: &[u8]) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn write_hash(data: &[u8], path: &String, data_type: &String) -> Result<(), String> {
    let obj_file = File::create(path);

    let data_header = data_type.as_bytes();

    let buf = [data_header, &[SEPARATOR], data].concat();
    if let Err(_) = obj_file {
        return Err("Error creating file".to_string());
    }
    if let Err(_) = obj_file.unwrap().write_all(&buf) {
        return Err("Error writing to file".to_string());
    }
    Ok(())
}

// TODO: find better way to do this
// path == None for normal usage
pub fn cat_file(hash: &String, path_prefix: &String) -> Result<[Vec<u8>; 2], String> {
    let path = path_prefix.clone() + hash.as_str();
    let file_bytes = fs::read(&path);

    match file_bytes {
        Err(e) => {
            Err(String::from("Error reading file at ") + path.as_str() + e.to_string().as_str())
        }
        Ok(data) => {
            // split the bytes at the separator
            let mut iter = data.split(|&x| x == SEPARATOR.into());
            let data_type = iter.next().expect("Error reading file");
            let file_data = iter.collect::<Vec<&[u8]>>().concat();
            // data type will always be valid ascii;

            return Ok([data_type.to_vec(), file_data]);
        }
    }
}

pub fn hash_file(path: &String) -> Result<(), String> {
    let file_data = fs::read(path);

    match file_data {
        Err(_) => Err(String::from("Error reading file at") + path.as_str()),
        Ok(data) => {
            let hash = hash_obj(&data);
            let hash_path = "./.yeet/objects/".to_string() + hash.to_string().as_str();

            // TODO: check obj type
            if let Err(e) = write_hash(&data, &hash_path, &String::from("blob")) {
                return Err(e);
            } else {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::yeet::hash_obj;
    use std::fs;

    use super::{cat_file, write_hash};

    #[test]
    fn hash_works() {
        let a = String::from("new random string");
        let b = String::from("new random string");

        assert_eq!(hash_obj(a.as_bytes()), hash_obj(b.as_bytes()));
    }

    #[test]
    fn hash_writing_works() {
        if let Err(_) = fs::create_dir("./.tests/hash_writing_works") {
            assert!(false, "failed to create test dir");
        }
        let a = String::from("new random string");

        let a_hash = hash_obj(a.as_bytes());

        let a_path = String::from("./.tests/hash_writing_works") + a_hash.to_string().as_str();

        // write hashes of both strings
        if let Err(_) = write_hash(a.as_bytes(), &a_path, &String::from("blob")) {
            assert!(false, "failed to write a hash");
        }
    }

    #[test]
    fn catfile_works() {
        // write a hash first
        if let Err(_) = fs::create_dir("./.tests/catfile_works") {
            assert!(false, "failed to create test dir");
        }
        let a = String::from("new random string");

        let a_hash = hash_obj(a.as_bytes());

        let a_path = String::from("./.tests/catfile_works") + a_hash.to_string().as_str();

        // write hashes of both strings
        if let Err(_) = write_hash(a.as_bytes(), &a_path, &String::from("blob")) {
            assert!(false, "failed to write a hash");
        }

        let a_res = cat_file(&a_hash.to_string(), &String::from("./.tests/catfile_works"));

        if let Ok(a) = a_res {
            let a_data_type = String::from_utf8(a[0].clone()).expect("fail data-type parse for a");
            assert_eq!(a_data_type, String::from("blob"));

            let a_file_data = a[1].clone();
            assert_eq!(a_file_data, String::from("new random string").as_bytes());
        } else {
            assert!(false, "catfile a failed");
        }
    }
}
