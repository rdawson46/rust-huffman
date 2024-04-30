mod minheap;
mod decoder;

use minheap::MinHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // get file from file tree
    // open file
    // run MapReduce on chars
    // pass to minHeap

    let filename = std::env::args().nth(1);

    if let Some(filename) = filename {
        // convert to path
        let path = std::path::Path::new(&filename);

        if path.exists() || path.is_file() {
            let file = File::open(&path);

            match file {
                Ok(file) => {
                    // file open
                    // parse chars

                    let reader = BufReader::new(file);
                    let mut chars: Vec<char> = vec![];

                    // collect chars
                    for line in reader.lines() {
                        for c in line.unwrap_or(String::from("\n")).chars() {
                            chars.push(c);
                        }
                    }

                    let mh = MinHeap::new(chars);

                    let encoding = mh.encode();
                    println!("{encoding}");
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        } else {
            println!("File cannot be found");
        }
    } else {
        println!("No file passed")
    }
}
