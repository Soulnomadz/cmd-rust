use assert_fs::prelude::*;

fn main() {
    let temp = assert_fs::TempDir::new().unwrap();
    println!("{}", temp.path().display());
    println!("{}", temp.child("foo/bar.txt").display());
    temp.close().unwrap();
}