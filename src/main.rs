use std::fs;
use std::fs::File;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::Path;

use std::io::{self, BufRead};

// use std::env;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-naive-approach
fn read_lines_naive(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-naive-approach
fn read_lines_naive_concise(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines_efficient<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html#reading-a-file
fn book(file_path: &Path) {
    // --snip--
    let display = file_path.display();
    println!("In file {display}");

    let contents = read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html#open
fn rust_by_exemple(file_path: &Path) -> Vec<u8> {
    // Create a path to the desired file
    let display = file_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(size) => print!("{} contains: {} bytes\n", display, size),
    }

    // // Read the file contents into a string, returns `io::Result<usize>`
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }

    // `file` goes out of scope, and the "hello.txt" file gets closed
    buffer
}

fn get_header(byte: &[u8]) -> &[u8] {
    let header_size = byte[0];
    println!("Header length: {}", header_size);

    &byte[..header_size as usize]
}

fn get_data_reords(byte: &[u8]) -> &[u8] {
    let header_size = byte[0]; // & ?

    let data_reords_size = u32::from_le_bytes(byte[4..8].try_into().unwrap());
    println!("Data Record Size: {}", data_reords_size);

    &byte[header_size as usize..data_reords_size as usize]
}

fn main() {
    let swimming_path = Path::new("resources/swimming.fit");

    let content = fs::read(swimming_path).expect("Failure");

    let header = get_header(&content);
    println!("Protocol Version: {}", header[1]);

    let profile_version = u16::from_le_bytes(header[2..4].try_into().unwrap());
    println!("Profile Version: {}", profile_version);

    match str::from_utf8(&header[8..12]) {
        // ASCII ?
        Ok(a) => println!("{}", a),
        Err(e) => eprintln!("{}", e),
    }
    // let res = String::from_utf8_lossy(&header[7..12]);
    // println!("{}", res);

    let data_reords = get_data_reords(&content);

    let record_header = &data_reords[0];

    for i in 0..8 {
        let bit = (record_header >> i) & 1; // on décale i positions puis on masque
        println!("Bit {} = {}", i, bit);
    }
}
