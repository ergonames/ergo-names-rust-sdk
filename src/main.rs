use anyhow::{Result};
use serde_json::{Value};

const EXPLORER_API_URL: &str = "https://api-testnet.ergoplatform.com/";

// const MINT_ADDRESS: &str = "3WwKzFjZGrtKAV7qSCoJsZK9iJhLLrUa3uwd4yw52bVtDVv6j5TL";

// enum Token {
//     Id,
//     BoxId,
//     Name,
// }

fn get_token_data(token_name: &str) -> Result<Value> {
    let mut url: String = EXPLORER_API_URL.to_owned();
    url.push_str("api/v1/tokens/search?query=");
    url.push_str(token_name);
    println!("{}", url);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn create_token_data(token_name: &str) -> Result<f64> {
    let total: f64 = get_token_data(&token_name).unwrap()["total"].to_owned().as_f64().unwrap();
    let needed_calls: i64 = ((total / 500.0) + 1.0).floor() as i64;
    let mut offset = 0;
    let mut transaction_data: String = "".to_owned();
    for i in needed_calls {

    }
    return Ok(total.to_owned());
}

fn reformat_name_search(name: &str) -> String {
    let mut new_name: String = "".to_owned();
    for c in name.chars() {
        if c == ' ' {
            new_name.push_str("%20");
        } else {
            new_name.push(c);
        }
    }
    return new_name;
}

pub fn resolve_ergoname(name: &str) -> Result<f64> {
    let refactored_name: String = reformat_name_search(name);
    //let token_data: Value = get_token_data(&refactored_name).unwrap();
    let total = create_token_data(&refactored_name);
    return total;
}

fn main() {
    let name: &str = "test mint v0.1.1";
    let address: f64 = resolve_ergoname(name).unwrap();
    // let d = serde_json::to_string(&address);
    println!("{}", address.to_string());
}