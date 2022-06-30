use anyhow::{Result};
use serde_json::{Value};

use crate::core_funcs::check_name_valid;
use crate::endpoints;
use crate::endpoints::get_address_confirmed_balance;
use crate::types;
use crate::consts;
use crate::utils;

pub fn create_token_data(token_name: &str) -> Result<String> {
    let total: u64 = endpoints::get_token_data(&token_name, 1, 0).unwrap()["total"].to_owned().as_u64().unwrap();
    let needed_calls: u64 = (total / 500) + 1;
    let mut offset: u64 = 0;
    let mut transaction_data: String = "".to_owned();
    if total > 0 {
        for _i in 0..needed_calls {
            transaction_data = transaction_data + &endpoints::get_token_data(&token_name, 500, offset).unwrap()["items"].to_string();
            offset = offset + 500;
        }
        return Ok(transaction_data);
    } else {
        return Ok("None".to_string());
    }
}

pub fn create_token_vector(data: String) -> Vec<types::Token> {
    let data_value: Value = serde_json::from_str(&data).unwrap();
    let mut token_vector: Vec<types::Token> = Vec::new();
    for i in 0..data_value.as_array().unwrap().len() {
        let raw = data_value.get(i).unwrap();
        let tk: types::Token = types::Token {
            name:String::from(utils::remove_quotes(raw["name"].to_string())),
            id:String::from(utils::remove_quotes(raw["id"].to_string())),
            box_id:String::from(utils::remove_quotes(raw["boxId"].to_string())),
        };
        token_vector.push(tk);
    }
    return token_vector
}

pub fn get_asset_minted_at_address(token_vector: Vec<types::Token>) -> String{
    for i in token_vector {
        let address: String = get_box_address(&i.box_id);
        if address == consts::MINT_ADDRESS.to_owned() {
            return i.id;
        }
    }
    return "None".to_owned();
}

pub fn get_box_address(box_id: &str) -> String {
    let box_data: Value = endpoints::get_box_by_id(box_id).unwrap();
    let address: String = utils::remove_quotes(box_data["address"].to_string());
    return address;
}

pub fn get_max_transactions_for_token(token_id: &str) -> u64 {
    let data: Value = endpoints::get_single_transactions_for_token(token_id).unwrap();
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
    return utils::remove_quotes(box_id);
}

pub fn get_address_tokens(address: &str) -> Vec<Value> {
    let balance: Value = get_address_confirmed_balance(address).unwrap();
    let tokens: &Vec<Value> = &balance["tokens"].as_array().unwrap().to_owned();
    return tokens.to_owned();
}

pub fn convert_to_token_array(data: Vec<Value>) -> Vec<types::Token> {
    let mut token_vector: Vec<types::Token> = Vec::new();
    for i in 0..data.len() {
        let raw = data.get(i).unwrap();
        let tk: types::Token = types::Token {
            name:String::from(utils::remove_quotes(raw["name"].to_string())),
            id:String::from(utils::remove_quotes(raw["id"].to_string())),
            box_id:String::from(utils::remove_quotes(raw["boxId"].to_string())),
        };
        token_vector.push(tk);
    }
    return token_vector;
}

pub fn remove_invalid_tokens(token_vector: Vec<types::Token>) -> Vec<types::Token> {
    let mut new_token_vector: Vec<types::Token> = Vec::new();
    for i in 0..token_vector.len() {
        if check_name_valid(&token_vector.get(i).unwrap().name) {
            let tk = types::Token {
                name: token_vector.get(i).unwrap().name.to_string(),
                id: token_vector.get(i).unwrap().id.to_string(),
                box_id: token_vector.get(i).unwrap().box_id.to_string(),
            };
            new_token_vector.push(tk);
        }
    }
    return new_token_vector;
}

pub fn check_correct_ownership(token_vector: Vec<types::Token>, user_address: &str) -> Vec<types::Token> {
    let mut new_token_vector: Vec<types::Token> = Vec::new();
    for i in 0..token_vector.len() {
        if token_vector.get(i).unwrap().box_id == user_address {
            let tk = types::Token {
                name: token_vector.get(i).unwrap().name.to_string(),
                id: token_vector.get(i).unwrap().id.to_string(),
                box_id: token_vector.get(i).unwrap().box_id.to_string(),
            };
            new_token_vector.push(tk);
        }
    }
    return token_vector;
}

pub fn get_first_transaction(transactions_data: Value) -> Value {
    let first: &Value = transactions_data.get(0).unwrap();
    return first.to_owned();
}

pub fn get_block_id_from_transaction(transaction_data: Value) -> String {
    let block_id: String = transaction_data["items"][0]["headerId"].to_string();
    return utils::remove_quotes(block_id);
}

pub fn get_height_from_transaction(block_id: &str) -> String {
    let block_data: Value = endpoints::get_block_by_id(block_id).unwrap();
    let height: String = block_data["block"]["header"]["height"].to_string();
    return utils::remove_quotes(height);
}

pub fn get_timestamp_from_transaction(block_id: &str) -> String {
    let block_data: Value = endpoints::get_block_by_id(block_id).unwrap();
    let timestamp: String = block_data["block"]["header"]["timestamp"].to_string();
    return utils::remove_quotes(timestamp);
}