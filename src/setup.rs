extern crate reqwest;

// use reqwest;
use std::io::Read;
use std::io::Write;
type Error = Box<dyn std::error::Error + Send + Sync>;

const CODES: &str = "https://raw.githubusercontent.com/i1Fury/CodeRaidBot/master/bin/codes.txt";

fn to_err(msg: &str) -> Result<(String, Vec<String>), Error> {
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg)))
}

pub async fn setup() -> Result<(String, Vec<String>), Error> {
    // return error for testing
    // return to_err("test");

    // Ask the user for their token if one is not an env variable nor saved in token.txt
    let token = std::env::var("DISCORD_TOKEN").unwrap_or_else(|_| {
        // check if the file exists
        if std::path::Path::new("token.txt").exists() {
            // ask the user if they want to use the saved token or enter a new one
            print!("Use saved token? (y/n) ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            // if the user wants to use the saved token, read it from the file
            if input.to_lowercase().trim() == "y" {
                let mut file = std::fs::File::open("token.txt").unwrap();
                let mut token = String::new();
                file.read_to_string(&mut token).unwrap();
                return token.trim().to_string();
            }
        }

        // if the file does not exist, or the user does not want to use the saved token, ask for a new token
        println!("Please enter your Discord token:");
        let mut token = String::new();
        std::io::stdin().read_line(&mut token).unwrap();
        token.pop();
        token = token.trim().to_string();

        // Ask the user if they want to save the token to a file
        print!("Save token to file? (y/n) ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase().trim() == "y" {
            let mut file = std::fs::File::create("token.txt").unwrap_or_else(|_| {
                panic!("Failed to create token.txt");
            });
            file.write_all(token.as_bytes()).unwrap_or_else(|_| {
                panic!("Failed to write token to token.txt");
            });
        }
        token
    });

    // Ask if they want to use the default codes list text file or enter a path to their own
    print!("Use default codes list? (y/n) ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let codes_list_path = if input.to_lowercase().trim() == "y" {
        // check if codes.txt exists
        if std::path::Path::new("codes.txt").exists() {
            // if it does, use it
            "codes.txt".to_string()
        } else {
            // if it does not, download it from the repo and use it
            let resp = match reqwest::get(CODES).await {
                Ok(resp) => resp.text().await.unwrap_or_else(|_| {
                    panic!("Failed to download codes.txt");
                }),
                Err(e) => {
                    panic!("Failed to download codes.txt: {}", e);
                }
            };

            // create the file
            let mut file = std::fs::File::create("codes.txt").unwrap_or_else(|_| {
                panic!("Failed to create codes.txt");
            });

            // write the codes to the file
            file.write_all(resp.as_ref()).unwrap_or_else(|_| {
                panic!("Failed to write the codes to codes.txt");
            });
            "codes.txt".to_string()
        }
    } else {
        print!("Enter path to codes list: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop();
        input = input.trim().to_string();
        // check if the file exists
        if std::path::Path::new(&input).exists() {
            input.to_string()
        } else {
            panic!("File does not exist");
        }
    };
    // turn the codes list into a vector of strings
    let mut codes_list: Vec<String> = Vec::new();
    let mut file = std::fs::File::open(codes_list_path).unwrap_or_else(|_| {
        panic!("Failed to open codes list");
    });
    let mut codes = String::new();
    file.read_to_string(&mut codes).unwrap_or_else(|_| {
        panic!("Failed to read codes list");
    });
    for line in codes.lines() {
        codes_list.push(line.to_string());
    }

    // return the token and the codes list
    Ok((token, codes_list))
}