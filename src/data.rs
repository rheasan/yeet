use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;

const SEPARATOR: u8 = 0x00u8;

fn hash_obj(t: &[u8]) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn write_data(data: &[u8], path: &String, data_type: &String) -> Result<(), String> {
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
pub fn get_data(hash: &String, path_prefix: String) -> Result<[Vec<u8>; 2], String> {
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

pub fn save_hash(path: &String) -> Result<(), String> {
    let file_data = fs::read(path);

    match file_data {
        Err(_) => Err(String::from("Error reading file at") + path.as_str()),
        Ok(data) => {
            let hash = hash_obj(&data);
            let hash_path = "./.yeet/objects/".to_string() + hash.to_string().as_str();

            // TODO: check obj type
            if let Err(e) = write_data(&data, &hash_path, &String::from("blob")) {
                return Err(e);
            } else {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::hash_obj;
    use std::fs;

    use super::{get_data, write_data};

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
        if let Err(_) = write_data(a.as_bytes(), &a_path, &String::from("blob")) {
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
        if let Err(_) = write_data(a.as_bytes(), &a_path, &String::from("blob")) {
            assert!(false, "failed to write a hash");
        }

        let a_res = get_data(&a_hash.to_string(), String::from("./.tests/catfile_works"));

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
