// src/shell.rs

use crate::filesystem::{u32_to_string, FileSystem};

pub fn execute_shell_commands(fs: &mut FileSystem) {
    // 创建文件
    let file_name = "hello.txt";
    if let Ok(inode_index) = fs.create_file(file_name) {
        println!("Created file with inode: {}", inode_index);

        // 写入文件
        let data = b"Hello, World!";
        if let Err(e) = fs.write_file(inode_index, data) {
            println!("Error writing file: {}", e);
        }

        // 读取文件
        if let Some(content) = fs.read_file(inode_index) {
            println!(
                "File content: {}",
                core::str::from_utf8(content).unwrap_or("")
            );
        } else {
            println!("Failed to read file.");
        }
    }

    // 列出文件
    fs.list_files();
}
