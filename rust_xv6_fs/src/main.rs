mod filesystem;
mod shell;

use crate::filesystem::FileSystem;
use crate::shell::execute_shell_commands;

fn main() {
    // 添加一些调试信息
    println!("Starting the file system...");

    let mut fs = FileSystem::new(1024 * 1024, 512, 100);
    execute_shell_commands(&mut fs);

    // 再加一个打印，确认执行完毕
    println!("File system operations complete.");
}
