use anyhow::{Result};
use serde_json::{Value};

const EXPLORER_API_URL: &str = "https://api-testnet.ergoplatform.com/";

const MINT_ADDRESS: &str = "3WwKzFjZGrtKAV7qSCoJsZK9iJhLLrUa3uwd4yw52bVtDVv6j5TL";

struct  Token {
    id: String,
    box_id: String,
}

fn get_token_data(token_name: &str, limit: u64, offset: u64) -> Result<Value> {
    let mut url: String = EXPLORER_API_URL.to_owned();
    url.push_str("api/v1/tokens/search?query=");
    url.push_str(token_name);
    url.push_str("&limit=");
    url.push_str(&limit.to_string());
    url.push_str("&offset=");
    url.push_str(&offset.to_string());
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn create_token_data(token_name: &str) -> Result<String> {
    let total: u64 = get_token_data(&token_name, 1, 0).unwrap()["total"].to_owned().as_u64().unwrap();
    let needed_calls: u64 = (total / 500) + 1;
    let mut offset: u64 = 0;
    let mut transaction_data: String = "".to_owned();

    for _i in 0..needed_calls {
        transaction_data = transaction_data + &get_token_data(&token_name, 500, offset).unwrap()["items"].to_string();
        offset = offset + 500;
    }
    return Ok(transaction_data);
}

fn create_token_vector(data: String) -> Vec<Token> {
    let data_value: Value = serde_json::from_str(&data).unwrap();
    let mut token_vector: Vec<Token> = Vec::new();
    for i in 0..data_value.as_array().unwrap().len() {
        let raw = data_value.get(i).unwrap();
        let tk: Token = Token {
            id:String::from(remove_quotes(raw["id"].to_string())),
            box_id:String::from(remove_quotes(raw["boxId"].to_string())),
        };
        token_vector.push(tk);
    }
    return token_vector
}

fn get_asset_minted_at_address(token_vector: Vec<Token>) -> String{
    for i in token_vector {
        let address: String = get_box_address(&i.box_id);
        if address == MINT_ADDRESS.to_owned() {
            return i.id;
        }
    }
    return "None".to_owned();
}

fn get_box_by_id(box_id: &str) -> Result<Value> {
    let mut url: String = EXPLORER_API_URL.to_owned();
    url.push_str("api/v1/boxes/");
    url.push_str(box_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn get_box_address(box_id: &str) -> String {
    let box_data: Value = get_box_by_id(box_id).unwrap();
    let address: String = remove_quotes(box_data["address"].to_string());
    return address;
}

fn get_token_transaction_data(token_id: &str) -> Result<Value> {
    let total: u64 = get_max_transactions_for_token(token_id);
    let mut url: String = EXPLORER_API_URL.to_owned();
    url.push_str("api/v1/assets/search/byTokenId?query=");
    url.push_str(token_id);
    url.push_str("&offset=");
    url.push_str(&(total-1).to_string());
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data["items"].to_owned());
}

fn get_single_transactions_for_token(token_id: &str) -> Result<Value> {
    let mut url: String = EXPLORER_API_URL.to_owned();
    url.push_str("api/v1/assets/search/byTokenId?query=");
    url.push_str(token_id);
    url.push_str("&limit=1");
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn get_max_transactions_for_token(token_id: &str) -> u64 {
    let data: Value = get_single_transactions_for_token(token_id).unwrap();
    let total: u64 = data["total"].as_u64().unwrap();
    return total;
}

fn get_last_transaction(data: Value) -> Result<Value> {
    let length: usize = data.as_array().unwrap().len();
    let last_borrowed: &Value = &data.get(length-1).unwrap();
    let last: Value = last_borrowed.to_owned();
    return Ok(last);
}

fn get_box_id_from_token_data(data: Value) -> String{
    let box_id: String = data["boxId"].to_string();
    return remove_quotes(box_id);
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

pub fn remove_quotes(i_str: String) -> String {
    let n_str: String = i_str.replace('"', "");
    return n_str;
}

pub fn resolve_ergoname(name: &str) -> String {
    let refactored_name: String = reformat_name_search(name);
    let token_data: String = create_token_data(&refactored_name).unwrap();
    let token_vector: Vec<Token> = create_token_vector(token_data);
    let token_id: String = get_asset_minted_at_address(token_vector);
    let token_transactions: Value = get_token_transaction_data(&token_id).unwrap();
    let token_last_transaction: Value = get_last_transaction(token_transactions).unwrap();
    let token_current_box_id: String = get_box_id_from_token_data(token_last_transaction);
    let address: String = get_box_address(&token_current_box_id);
    return address;
}