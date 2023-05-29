use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::PathBuf;

const SEPARATOR: u8 = 0x00u8;

pub struct FileData {
    pub file_name: String,
    pub file_type: String,
    pub hash: u64,
}

pub fn hash_dir(data: &Vec<FileData>) -> u64 {
    let strings = data
        .iter()
        .map(|x| {
            return format!("{} {} {}", x.file_type, x.hash, x.file_name);
        })
        .collect::<Vec<String>>()
        .join("\n");

    return write_obj_hash(strings.as_bytes(), "tree".to_string());
}

pub fn write_obj_hash(data: &[u8], type_: String) -> u64 {
    let mut s = DefaultHasher::new();
    data.hash(&mut s);
    let hash = s.finish();

    let buf = [type_.as_bytes(), &[SEPARATOR], data].concat();

    let mut obj_file = File::create(PathBuf::from("./.yeet/objects").join(hash.to_string()))
        .expect("Error creating file");

    obj_file
        .write_all(buf.as_slice())
        .expect("Error writing to file");

    return hash;
}

// TODO: find better way to do this
pub fn get_data(hash: &String, path_prefix: String) -> Result<[Vec<u8>; 2], String> {
    let file_path = PathBuf::from(path_prefix).join(hash);

    let file_bytes = fs::read(&file_path);
    if let Err(_) = file_bytes {
        return Err(format!("Error reading file at {:?}", file_path.as_os_str()));
    }

    let file_bytes = file_bytes.unwrap();
    let mut bytes = file_bytes.split(|&x| x == SEPARATOR);
    let file_type = bytes.next().unwrap().to_vec();
    let file_data = bytes.collect::<Vec<_>>().concat();

    Ok([file_type, file_data])
}

/*
#[cfg(test)]
mod tests {
    use crate::data::hash_obj;
    use std::{fs, path::PathBuf};

    use super::{get_data, write_data};

    #[test]
    fn hash_works() {
        let a = String::from("new random string");
        let b = String::from("new random string");

        assert_eq!(hash_obj(a.as_bytes()), hash_obj(b.as_bytes()));
    }

    #[test]
    fn hash_writing_works() {
        if let Err(_) = fs::create_dir("./tests/hash_writing_works") {
            assert!(false, "failed to create test dir");
        }
        let a = String::from("new random string");

        let a_hash = hash_obj(a.as_bytes());

        let a_path = PathBuf::from("./tests/hash_writing_works/").join(a_hash.to_string());

        // write hashes of both strings
        if let Err(_) = write_data(a.as_bytes(), &a_path, &String::from("blob")) {
            assert!(false, "failed to write a hash");
        }
    }

    #[test]
    fn catfile_works() {
        // write a hash first
        if let Err(_) = fs::create_dir("./tests/catfile_works") {
            assert!(false, "failed to create test dir");
        }
        let a = String::from("new random string");

        let a_hash = hash_obj(a.as_bytes());

        let a_path = PathBuf::from("./tests/catfile_works/").join(a_hash.to_string());

        // write hashes of both strings
        write_data(a.as_bytes(), &a_path, &String::from("blob")).expect("Error writing hash");

        let a_res = get_data(&a_hash.to_string(), String::from("./tests/catfile_works"));

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
*/
