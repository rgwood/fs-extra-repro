
use std::{fs, path::Path};

fn main() {
    fs::create_dir("Test").unwrap();
    println!("Created a directory named 'Test'");
    check_if_path_exists("Test");

    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    fs_extra::dir::move_dir("Test", "test", &options).unwrap();

    println!("Called fs_extra::dir::move_dir() to rename 'Test' to 'test'");
    check_if_path_exists("Test");
    check_if_path_exists("test");
}

fn check_if_path_exists(dir_name: &str) {
    let exists = Path::new(dir_name).exists();
    println!("Path '{dir_name}' exists: {exists}");
}
