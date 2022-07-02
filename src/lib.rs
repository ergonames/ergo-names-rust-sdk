use anyhow::{Result};
use serde_json::{Value};
use chrono::prelude::*;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub struct  Token {
    pub name: String,
    pub id: String,
    pub box_id: String,
}

pub const EXPLORER_API_URL: &str = "https://api-testnet.ergoplatform.com/";
pub const MINT_ADDRESS: &str = "3WwKzFjZGrtKAV7qSCoJsZK9iJhLLrUa3uwd4yw52bVtDVv6j5TL";

pub fn check_name_valid(name: &str) -> bool {
    for c in name.chars() {
        let char_code: u8 = c as u8;
        if char_code <= 44 {
            return false;
        } else if char_code == 47 {
            return false;
        } else if char_code >= 58 && char_code <= 94 {
            return false;
        } else if char_code == 96 {
            return false;
        } else if char_code >= 123 && char_code <= 125 {
            return false;
        } else if char_code >= 127 {
            return false;
        }
    }
    return true;
}

pub fn reformat_name(name: &str) -> String {
    return name.to_lowercase();
}

pub fn check_name_price(name: &str) -> String {
    let _: String = reformat_name(name);
    return "None".to_owned();
}

pub fn resolve_ergoname(name: &str) -> String {
    let token_data: String = create_token_data(&name).unwrap();
    if token_data != "None" {
        let token_vector: Vec<Token> = create_token_vector(token_data);
        let token_id: String = get_asset_minted_at_address(token_vector);
        let token_transactions: Value = get_token_transaction_data(&token_id).unwrap();
        let token_last_transaction: Value = get_last_transaction(token_transactions).unwrap();
        let token_current_box_id: String = get_box_id_from_token_data(token_last_transaction);
        let address: String = get_box_address(&token_current_box_id);
        return address;
    } else {
        return "None".to_owned();
    }
}

pub fn check_already_registered(name: &str) -> bool {
    let address: String = resolve_ergoname(name);
    if address != "None" {
        return true;
    } else {
        return false;
    }
}

pub fn reverse_search(address: &str) -> Vec<Token> {
    let token_data: Vec<Value> = get_address_tokens(address);
    let token_vector: Vec<Token> = convert_to_token_array(token_data);
    let valid_names_vector: Vec<Token> = remove_invalid_tokens(token_vector);
    let owned_vector: Vec<Token> = check_correct_ownership(valid_names_vector, address);
    return owned_vector;
}

pub fn get_total_amount_owned(address: &str) -> u32 {
    let token_vector: Vec<Token> = reverse_search(address);
    let total_amount: u32 = token_vector.len() as u32;
    return total_amount;
}

pub fn get_block_id_registered(name: &str) -> String {
    let token_data: String = create_token_data(&name).unwrap();
    let token_vector: Vec<Token> = create_token_vector(token_data);
    let token_id: String = get_asset_minted_at_address(token_vector);
    let first_transaction: Value = get_single_transaction_by_token_id(&token_id).unwrap();
    let block_id: String = get_block_id_from_transaction(first_transaction);
    return block_id;
}

pub fn get_block_registered(name: &str) -> String {
    let block_id: String = get_block_id_registered(name);
    let height: String = get_height_from_transaction(&block_id);
    return height;
}

pub fn get_timestamp_registered(name: &str) -> u64 {
    let block_id: String = get_block_id_registered(name);
    let timestamp: String = get_timestamp_from_transaction(&block_id);
    return timestamp.parse::<u64>().unwrap();
}

pub fn get_date_registerd(name: &str) -> String {
    let timestamp: u64 = get_timestamp_registered(name);
    let reformated_time: SystemTime = UNIX_EPOCH + Duration::from_millis(timestamp);
    let datetime: DateTime<Utc> = DateTime::<Utc>::from(reformated_time);
    let timestamp_str: String = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    return timestamp_str;
}

fn remove_quotes(i_str: String) -> String {
    let n_str: String = i_str.replace('"', "");
    return n_str;
}

fn get_token_data(token_name: &str, limit: u64, offset: u64) -> Result<Value> {
    let url: String = format!("{}api/v1/tokens/search?query={}&limit={}&offset={}", EXPLORER_API_URL, token_name, limit, offset);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn get_box_by_id(box_id: &str) -> Result<Value> {
    let url: String = format!("{}api/v1/boxes/{}", EXPLORER_API_URL, box_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn get_block_by_id(block_id: &str) -> Result<Value> {
    let url: String = format!("{}api/v1/blocks/{}", EXPLORER_API_URL, block_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn get_token_transaction_data(token_id: &str) -> Result<Value> {
    let total: u64 = get_max_transactions_for_token(token_id);
    let url: String = format!("{}api/v1/tokens/search?query={}&offset={}", EXPLORER_API_URL, token_id, total-1);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data["items"].to_owned());
}

fn get_single_transactions_for_token(token_id: &str) -> Result<Value> {
    let url: String = format!("{}api/v1/tokens/search/?query={}&limit=1", EXPLORER_API_URL, token_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn get_single_transaction_by_token_id(token_id: &str) -> Result<Value> {
    let url: String = format!("{}api/v1/assets/search/byTokenId?query={}&limit=1", EXPLORER_API_URL, token_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

fn get_address_confirmed_balance(address: &str) -> Result<Value> {
    let url: String = format!("{}api/v1/addresses/{}/balance/confirmed", EXPLORER_API_URL, address);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data)
}

pub fn create_token_data(token_name: &str) -> Result<String> {
    let total: u64 = get_token_data(&token_name, 1, 0).unwrap()["total"].to_owned().as_u64().unwrap();
    let needed_calls: u64 = (total / 500) + 1;
    let mut offset: u64 = 0;
    let mut transaction_data: String = "".to_owned();
    if total > 0 {
        for _i in 0..needed_calls {
            transaction_data = transaction_data + &get_token_data(&token_name, 500, offset).unwrap()["items"].to_string();
            offset = offset + 500;
        }
        return Ok(transaction_data);
    } else {
        return Ok("None".to_string());
    }
}

pub fn create_token_vector(data: String) -> Vec<Token> {
    let data_value: Value = serde_json::from_str(&data).unwrap();
    let mut token_vector: Vec<Token> = Vec::new();
    for i in 0..data_value.as_array().unwrap().len() {
        let raw = data_value.get(i).unwrap();
        let tk: Token = Token {
            name:String::from(remove_quotes(raw["name"].to_string())),
            id:String::from(remove_quotes(raw["id"].to_string())),
            box_id:String::from(remove_quotes(raw["boxId"].to_string())),
        };
        token_vector.push(tk);
    }
    return token_vector
}

pub fn get_asset_minted_at_address(token_vector: Vec<Token>) -> String{
    for i in token_vector {
        let address: String = get_box_address(&i.box_id);
        if address == MINT_ADDRESS.to_owned() {
            return i.id;
        }
    }
    return "None".to_owned();
}

pub fn get_box_address(box_id: &str) -> String {
    let box_data: Value = get_box_by_id(box_id).unwrap();
    let address: String = remove_quotes(box_data["address"].to_string());
    return address;
}

pub fn get_max_transactions_for_token(token_id: &str) -> u64 {
    let data: Value = get_single_transactions_for_token(token_id).unwrap();
    let total: u64 = data["total"].as_u64().unwrap();
    return total;
}

pub fn get_last_transaction(data: Value) -> Result<Value> {
    let length: usize = data.as_array().unwrap().len();
    let last_borrowed: &Value = &data.get(length-1).unwrap();
    let last: Value = last_borrowed.to_owned();
    return Ok(last);
}

pub fn get_box_id_from_token_data(data: Value) -> String{
    let box_id: String = data["boxId"].to_string();
    return remove_quotes(box_id);
}

pub fn get_address_tokens(address: &str) -> Vec<Value> {
    let balance: Value = get_address_confirmed_balance(address).unwrap();
    let tokens: &Vec<Value> = &balance["tokens"].as_array().unwrap().to_owned();
    return tokens.to_owned();
}

pub fn convert_to_token_array(data: Vec<Value>) -> Vec<Token> {
    let mut token_vector: Vec<Token> = Vec::new();
    for i in 0..data.len() {
        let raw = data.get(i).unwrap();
        let tk: Token = Token {
            name:String::from(remove_quotes(raw["name"].to_string())),
            id:String::from(remove_quotes(raw["id"].to_string())),
            box_id:String::from(remove_quotes(raw["boxId"].to_string())),
        };
        token_vector.push(tk);
    }
    return token_vector;
}

pub fn remove_invalid_tokens(token_vector: Vec<Token>) -> Vec<Token> {
    let mut new_token_vector: Vec<Token> = Vec::new();
    for i in 0..token_vector.len() {
        if check_name_valid(&token_vector.get(i).unwrap().name) {
            let tk = Token {
                name: token_vector.get(i).unwrap().name.to_string(),
                id: token_vector.get(i).unwrap().id.to_string(),
                box_id: token_vector.get(i).unwrap().box_id.to_string(),
            };
            new_token_vector.push(tk);
        }
    }
    return new_token_vector;
}

pub fn check_correct_ownership(token_vector: Vec<Token>, user_address: &str) -> Vec<Token> {
    let mut new_token_vector: Vec<Token> = Vec::new();
    for i in 0..token_vector.len() {
        if token_vector.get(i).unwrap().box_id == user_address {
            let tk = Token {
                name: token_vector.get(i).unwrap().name.to_string(),
                id: token_vector.get(i).unwrap().id.to_string(),
                box_id: token_vector.get(i).unwrap().box_id.to_string(),
            };
            new_token_vector.push(tk);
        }
    }
    return token_vector;
}

fn get_block_id_from_transaction(transaction_data: Value) -> String {
    let block_id: String = transaction_data["items"][0]["headerId"].to_string();
    return remove_quotes(block_id);
}

fn get_height_from_transaction(block_id: &str) -> String {
    let block_data: Value = get_block_by_id(block_id).unwrap();
    let height: String = block_data["block"]["header"]["height"].to_string();
    return remove_quotes(height);
}

fn get_timestamp_from_transaction(block_id: &str) -> String {
    let block_data: Value = get_block_by_id(block_id).unwrap();
    let timestamp: String = block_data["block"]["header"]["timestamp"].to_string();
    return remove_quotes(timestamp);
}