use std::{fs::File};
use std::io::{BufReader, BufRead};

fn main() {
    FileLoader::load_wordlist();
}

struct FileLoader;

impl FileLoader {
    fn load_wordlist(){

        let file_result = File::open("./src/wordlist.txt");

        if let Ok(wordlist) = file_result {
            let reader = BufReader::new(wordlist);

            for line in reader.lines() {
                println!("{:?}", line.unwrap_or("".to_string()).trim().to_string());
            }

        } else if let Err(error) = file_result {
            println!("{:?}", error);
        }
    }
}

