use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Write;
use std::path::PathBuf;

const SEPARATOR: u8 = 0x00u8;
#[derive(Debug, PartialEq)]
pub enum ObjType {
    Blob,
    Tree,
    Commit,
}

pub struct FileData {
    pub file_name: String,
    pub file_type: String,
    pub hash: u64,
}

pub struct DirEntry {
    pub name: String,
    pub type_: ObjType,
    pub hash: String,
    pub path: PathBuf,
    pub children: Option<Vec<DirEntry>>,
}

impl DirEntry {
    fn new(
        name: String,
        type_: ObjType,
        hash: String,
        path: PathBuf,
        children: Option<Vec<DirEntry>>,
    ) -> DirEntry {
        DirEntry {
            name,
            type_,
            path,
            hash,
            children,
        }
    }
}

pub fn hash_dir(data: &Vec<FileData>) -> Result<u64, IOError> {
    let strings = data
        .iter()
        .map(|x| {
            return format!("{} {} {}", x.file_type, x.hash, x.file_name);
        })
        .collect::<Vec<String>>()
        .join("\n");

    return write_obj_hash(strings.as_bytes(), "tree".to_string());
}

pub fn write_obj_hash(data: &[u8], type_: String) -> Result<u64, IOError> {
    let mut s = DefaultHasher::new();
    data.hash(&mut s);
    let hash = s.finish();

    let buf = [type_.as_bytes(), &[SEPARATOR], data].concat();

    let mut obj_file = File::create(PathBuf::from("./.yeet/objects").join(hash.to_string()))?;

    obj_file.write_all(buf.as_slice())?;

    return Ok(hash);
}

// TODO: find better way to do this
pub fn get_data(hash: &String, path_prefix: String) -> Result<[Vec<u8>; 2], IOError> {
    let file_path = PathBuf::from(path_prefix).join(hash);

    let file_bytes = fs::read(&file_path)?;

    let mut bytes = file_bytes.split(|&x| x == SEPARATOR);
    let file_type = bytes.next().unwrap().to_vec();
    let file_data = bytes.collect::<Vec<_>>().concat();

    Ok([file_type, file_data])
}

fn decode_dir_data(hash: &String) -> Result<Vec<FileData>, IOError> {
    let [_, data] = get_data(hash, "./.yeet/objects".to_string())?;
    let strings = String::from_utf8(data.to_vec()).unwrap();
    let mut data: Vec<FileData> = Vec::new();
    for string in strings.split("\n") {
        let d = string
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if d.len() != 3 {
            return Err(IOError::new(
                IOErrorKind::InvalidData,
                format!("expected 3 values got {}", d.len()),
            ));
        } else {
            let file_type = d[0].clone();
            let hash = d[1].parse::<u64>().unwrap();
            let file_name = d[2].clone();
            if file_type.eq_ignore_ascii_case("tree") && file_type.eq_ignore_ascii_case("blob") {
                return Err(IOError::new(
                    IOErrorKind::InvalidData,
                    format!(
                        "Incorrect type found: {}. Expected 'tree' or 'blob', {:?}",
                        file_type, d
                    ),
                ));
            }

            let file_d = FileData {
                file_name,
                file_type,
                hash,
            };
            data.push(file_d);
        }
    }
    return Ok(data);
}

pub fn gen_tree(hash: String, name: String, path: PathBuf) -> Result<DirEntry, IOError> {
    let actual_hash = get_actual_hash(&hash)?;
    let dir_data = decode_dir_data(&actual_hash)?;
    if dir_data.is_empty() {
        return Ok(DirEntry::new(name, ObjType::Tree, actual_hash, path, None));
    }

    let children = dir_data
        .iter()
        .map(|x| {
            if x.file_type == "tree".to_string() {
                gen_tree(
                    x.hash.to_string(),
                    x.file_name.clone(),
                    path.join(x.file_name.clone()),
                )
                .unwrap()
            } else {
                return DirEntry::new(
                    x.file_name.clone(),
                    ObjType::Blob,
                    x.hash.to_string(),
                    path.join(x.file_name.clone()),
                    None,
                );
            }
        })
        .collect::<Vec<DirEntry>>();

    return Ok(DirEntry::new(
        name,
        ObjType::Tree,
        actual_hash,
        path,
        Some(children),
    ));
}

pub fn show_tree(entry: &DirEntry, count: usize) {
    let padding = String::from("\t").repeat(count);
    println!("{}Name: {}", padding, entry.name);
    println!("{}Type: {:?}", padding, entry.type_);
    println!("{}Path: {:?}", padding, entry.path);
    if let Some(children) = &entry.children {
        println!("{}children: ", padding);
        for i in children {
            show_tree(&i, count + 1);
        }
    }
}

pub fn write_entry(entry: DirEntry) {
    if entry.type_ == ObjType::Blob {
        let [_, file_data] = get_data(&entry.hash, String::from("./.yeet/objects/")).unwrap();
        let mut file = fs::File::create(entry.path).unwrap();
        file.write_all(file_data.as_slice()).unwrap();
    } else {
        fs::create_dir_all(entry.path).unwrap();
        if let Some(children) = entry.children {
            for i in children {
                write_entry(i);
            }
        }
    }
}

pub fn read_commit(hash: String) -> Result<(), IOError> {
    let actual_hash = get_actual_hash(&hash)?;
    if actual_hash == "initial" {
        return Err(IOError::new(
            IOErrorKind::Other,
            "No commits in current repo",
        ));
    }

    let [type_, data] = get_data(&actual_hash, "./.yeet/objects".to_string()).unwrap();
    if String::from_utf8(type_).unwrap() != "commit" {
        return Err(IOError::new(
            IOErrorKind::InvalidData,
            format!("Invalid commit or tag found {}", actual_hash),
        ));
    }
    let strings = String::from_utf8(data).unwrap();
    let mut data = strings.split("\n").map(|x| x.to_string());

    // TODO: fix this iter monster
    let _ = data.next().unwrap().split_once(" ").unwrap().1.to_string();
    let parent_hash = data.next().unwrap().split_once(" ").unwrap().1.to_string();
    let author_name = data.next().unwrap().split_once(" ").unwrap().1.to_string();
    let time = data.next().unwrap().split_once(" ").unwrap().1.to_string();
    let message = data.collect::<Vec<_>>().concat();

    println!("commit {}", actual_hash);
    println!("Author: {}", author_name);
    println!("Date: {}", time);
    println!("{}", message);

    if parent_hash != "initial" {
        print!("\n");
        read_commit(parent_hash)?;
    }

    Ok(())
}

pub fn get_commit_tree(hash: &String) -> Result<String, IOError> {
    let actual_hash = get_actual_hash(hash)?;
    let [type_, data] = get_data(&actual_hash, "./.yeet/objects".to_string())?;
    if String::from_utf8(type_).unwrap() != "commit" {
        return Err(IOError::new(
            IOErrorKind::InvalidData,
            format!("Invalid commit or hash : {}", hash),
        ));
    }
    let strings = String::from_utf8(data).unwrap();
    let mut commit_data = strings.split("\n").map(|x| x.to_string());
    let tree_hash = commit_data
        .next()
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .to_string();
    return Ok(tree_hash);
}

pub fn get_tag(tag: &String) -> Result<String, IOError> {
    let id = fs::read_to_string(PathBuf::from("./.yeet/refs/tags").join(&tag));
    return id;
}

pub fn set_tag(tag: String, hash: String) -> Result<(), IOError> {
    let actual_hash = get_actual_hash(&hash)?;
    let [type_, _] = get_data(&actual_hash, "./.yeet/objects".to_string())?;
    if String::from_utf8(type_).unwrap() != "commit" {
        return Err(IOError::new(
            IOErrorKind::InvalidData,
            "Invalid commit object found",
        ));
    }
    let mut tag_file = fs::File::create(PathBuf::from("./.yeet/refs/tags").join(&tag))?;

    tag_file.write(actual_hash.as_bytes())?;

    Ok(())
}

fn get_actual_hash(hash: &String) -> Result<String, IOError> {
    if let Err(_) = hash.parse::<u64>() {
        let actual_hash = get_tag(&hash)?;
        if actual_hash == "initial" {
            return Err(IOError::new(IOErrorKind::InvalidData, "No commits to tag"));
        }
        return Ok(actual_hash);
    } else {
        return Ok(hash.clone());
    }
}
