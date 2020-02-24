use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("we dont have the file yet"); // unwrap();
}
