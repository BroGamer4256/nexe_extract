use std::{fs, io::Write, path::PathBuf};

fn main() {
    let file = std::fs::read("/path/to/nexe").unwrap();
    let info_index = find_magic(
        &file,
        &[
            0x21, 0x28, 0x66, 0x75, 0x6E, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x28, 0x29, 0x20,
            0x7B, 0x70, 0x72, 0x6F, 0x63, 0x65, 0x73, 0x73, 0x2E, 0x5F, 0x5F, 0x6E, 0x65, 0x78,
            0x65, 0x20, 0x3D, 0x20, 0x7B, 0x22, 0x72, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65,
            0x73, 0x22, 0x3A, 0x7B,
        ],
    );
    let file_index = find_magic(
        &file,
        &[
            0x3B, 0x3B, 0x3B, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x28, 0x22, 0x6D, 0x6F,
            0x64, 0x75, 0x6C, 0x65, 0x22, 0x29, 0x2E, 0x72, 0x75, 0x6E, 0x4D, 0x61, 0x69, 0x6E,
            0x28, 0x29, 0x3B,
        ],
    );
    let file_info = file
        .iter()
        .skip(info_index)
        .take_while(|byte| byte != &&0x7D)
        .cloned()
        .collect::<Vec<_>>();
    let file_info = std::str::from_utf8(&file_info).unwrap();
    let file_info = parse_file_info(file_info);
    let files = &file.iter().skip(file_index).cloned().collect::<Vec<_>>();

    for file_info in file_info {
        let path = PathBuf::from(file_info.path);
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
        }
        let mut file = fs::File::create(path).unwrap();
        let index = file_info.index;
        let index_end = file_info.index + file_info.length;
        file.write(&files[index..index_end]).unwrap();
    }
}

#[derive(Default, Debug, Clone)]
struct FileInfo {
    path: String,
    index: usize,
    length: usize,
}

fn parse_file_info(raw: &str) -> Vec<FileInfo> {
    let mut files = vec![];
    let mut is_path = false;
    let mut is_index = false;
    let mut is_length = false;
    let mut buf = String::new();
    let mut file_info_buf = FileInfo::default();
    for c in raw.chars() {
        if c == '"' {
            if is_path {
                file_info_buf.path = buf.clone();
                buf.clear();
            }
            is_path = !is_path;
        } else if c == '[' {
            is_index = true;
        } else if c == ']' {
            is_length = false;
            file_info_buf.length = buf.parse().unwrap();
            buf.clear();
        } else if c == ',' {
            if is_index {
                is_index = false;
                is_length = true;
                file_info_buf.index = buf.parse().unwrap();
                buf.clear();
            } else {
                files.push(file_info_buf.clone());
            }
        } else if is_path || is_index || is_length {
            buf.push(c);
        }
    }
    files.push(file_info_buf.clone());

    files
}

fn find_magic(file: &Vec<u8>, magic: &[u8]) -> usize {
    let mut magic_index = 0;
    for (i, byte) in file.iter().enumerate() {
        if byte == &magic[magic_index] {
            magic_index = magic_index + 1;
            if magic_index == magic.len() {
                return i + 1;
            }
        } else {
            magic_index = 0;
        }
    }
    panic!();
}
