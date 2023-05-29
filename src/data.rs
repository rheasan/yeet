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
