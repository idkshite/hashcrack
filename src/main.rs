use std::{env, fs::File};
use std::error::Error;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let hash_to_crack = args.get(1).expect("No hash provided. Provide hash as first parameter");




    match FileLoader::load_wordlist() {
        Ok(words) => println!("Loaded {} words", words.len()),
        Err(e) => {
            eprintln!("Error loading wordlist: {}", e);
            // We can get more details about the error:
            eprintln!("Error kind: {:?}", e.kind());
        }
    }

    Ok(())
}

struct FileLoader;

impl FileLoader {
    fn load_wordlist() -> Result<Vec<String>, std::io::Error>{

        // this could be written with ? and without if let
        let file_result = File::open("./src/wordlist.txt");

        match file_result {
            Ok(wordlist) => {
                let reader = BufReader::new(wordlist);

                // reader.lines().map(|line_result| line_result.map(|line| line.to_string().trim()))
                reader.lines()
                    .map(|line_result| line_result.map(|l| l.trim().to_string()))
                    .collect()
            },
            Err(error) => {
                println!("{:?}", error);
                Err(error)
            }
        }
    }
}

