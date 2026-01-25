use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// use std::env;
use std::fs;

fn book(file_path: &Path) {
    // --snip--
    let display = file_path.display();
    println!("In file {display}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn rust_by_exemple(file_path: &Path) {
    // Create a path to the desired file
    let display = file_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(size) => print!("{} contains: {} bytes\n", display, size),
    }

    // // Read the file contents into a string, returns `io::Result<usize>`
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn main() {
    let swimming_path = Path::new("resources/swimming.fit");

    rust_by_exemple(swimming_path);
    // let display = swimming_path.display();

    // let mut file = match File::open(&swimming_path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }

    // print!("{}", s);
}
