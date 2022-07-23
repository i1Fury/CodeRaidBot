extern crate reqwest;

// use reqwest;
use std::io::Read;
use std::io::Write;
// type Error = Box<dyn std::error::Error + Send + Sync>;
use crate::Error;

const CODES: &str = "https://raw.githubusercontent.com/i1Fury/CodeRaidBot/master/bin/codes.txt";

async fn get_token() -> Result<String, Error> {
    let mut token = std::env::var("DISCORD_TOKEN").unwrap_or_default();

    if token.is_empty() {
        // check if the file exists
        if std::path::Path::new("token.txt").exists() {
            // ask the user if they want to use the saved token or enter a new one
            print!("Use saved token? (y/n) ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            // if the user wants to use the saved token, read it from the file
            if input.to_lowercase().trim() == "y" {
                let mut file = std::fs::File::open("token.txt")?;
                file.read_to_string(&mut token)?;
                return Ok(token.trim().to_string());
            }
        }

        // if the file does not exist, or the user does not want to use the saved token, ask for a new token
        println!("Please enter your Discord token:");
        std::io::stdin().read_line(&mut token)?;
        token.pop();
        token = token.trim().to_string();

        // Ask the user if they want to save the token to a file
        print!("Save token to file? (y/n) ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.to_lowercase().trim() == "y" {
            let mut file = std::fs::File::create("token.txt")?;
            file.write_all(token.as_bytes())?;
        }
    }
        
    return Ok(token);
}

async fn get_codes_list() -> Result<Vec<String>, Error> {
    // Ask if they want to use the default codes list text file or enter a path to their own
    print!("Use default codes list? (y/n) ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let codes_list_path;
    if input.to_lowercase().trim() == "y" {
        // check if codes.txt exists
        if std::path::Path::new("codes.txt").exists() {
            // if it does, use it
            codes_list_path = "codes.txt".to_string();
        } else {
            // if it does not, download it from the repo and use it
            let resp = reqwest::get(CODES).await?.text().await?;

            // create the file
            let mut file = std::fs::File::create("codes.txt")?;

            // write the codes to the file
            file.write_all(resp.as_ref())?;
            codes_list_path = "codes.txt".to_string();
        }
    } else {
        print!("Enter path to codes list: ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.pop();
        input = input.trim().to_string();
        // check if the file exists
        if std::path::Path::new(&input).exists() {
            codes_list_path = input.to_string();
        } else {
            return Err("File does not exist".into());
        }
    };
    // turn the codes list into a vector of strings
    let mut codes_list: Vec<String> = Vec::new();
    let mut file = std::fs::File::open(codes_list_path)?;
    let mut codes = String::new();
    file.read_to_string(&mut codes)?;
    for line in codes.lines() {
        codes_list.push(line.to_string());
    }

    return Ok(codes_list);
}

pub async fn setup() -> Result<(String, Vec<String>), Error> {

    // Ask the user for their token if one is not an env variable nor saved in token.txt
    let token = get_token().await?;
    // Ask the user for the codes list if one is not an env variable nor saved in codes.txt
    let codes_list = get_codes_list().await?;
    
    // return the token and the codes list
    Ok((token, codes_list))
}
