use anyhow::{Result};
use serde_json::{Value};

pub mod types;
pub mod endpoints;
pub mod consts;
pub mod utils;

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

pub fn resolve_ergoname(name: &str) -> String {
    let token_data: String = create_token_data(&name).unwrap();
    if token_data != "None" {
        let token_vector: Vec<types::Token> = create_token_vector(token_data);
        let token_id: String = get_asset_minted_at_address(token_vector);
        let token_transactions: Value = endpoints::get_token_transaction_data(&token_id).unwrap();
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

fn main() {
    let r = reformat_name("~balb");
    println!("{}", r);
}