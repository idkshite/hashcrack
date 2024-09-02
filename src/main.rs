mod errors;

use errors::HashError;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::{env, fs::File};
use sha1::digest::Output;
use sha1::{Digest, Sha1};

const SHA1_HEX_STRING_LENGTH:usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let hash_to_crack = args.get(1).expect("No hash provided. Provide hash as first parameter").trim();

    if hash_to_crack.chars().count() != SHA1_HEX_STRING_LENGTH {
        return Err(Box::new(HashError("sha1 hash is not valid".to_string())));
    }

    match FileLoader::load_wordlist() {
        Ok(passwords) => {
            println!("Loaded {} words", passwords.len());
            for password in passwords {
                let mut hasher = Sha1::new();
                hasher.update(password.clone().as_bytes());
                let as_sha1 =  hasher.clone().finalize();
                let as_sha1_hex =  &hex::encode(as_sha1);
                // println!("{:? } is {:?}", password.clone(), as_sha1_hex);
                if as_sha1_hex == hash_to_crack {
                    print!("The Password behind the hash is: {}", password);
                    return Ok(())
                }
            }

            println!("password not found");

        },
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

