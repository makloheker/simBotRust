// pemboy full senyum melihat ini

use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Deserialize)]
struct Response {
    message: Option<String>,
}

fn send_request(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post("https://api.simsimi.vn/v1/simtalk")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("text={}&lc=id", text))
        .send()?;

    if res.status().is_success() {
        let response: Response = res.json()?;
        Ok(response.message.unwrap_or_else(|| "no msg".to_string()))
    } else {
        Err(format!("err: {}", res.status()).into())
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    loop {
        print!("you>: ");
        stdout.flush().unwrap();
        
        let mut input_text = String::new();
        stdin.read_line(&mut input_text).unwrap();
        let input_text = input_text.trim();
        
        if ["exit", "quit", "keluar", "murtad"].contains(&input_text.to_lowercase().as_str()) {
            println!("enchat.");
            break;
        }

        match send_request(input_text) {
            Ok(message) => println!("bot>: {}", message),
            Err(e) => println!("{}", e),
        }
    }
}
