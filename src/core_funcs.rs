use serde_json::{Value};
use chrono::prelude::*;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::data_parsers;
use crate::endpoints;
use crate::types;

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
    let token_data: String = data_parsers::create_token_data(&name).unwrap();
    if token_data != "None" {
        let token_vector: Vec<types::Token> = data_parsers::create_token_vector(token_data);
        let token_id: String = data_parsers::get_asset_minted_at_address(token_vector);
        let token_transactions: Value = endpoints::get_token_transaction_data(&token_id).unwrap();
        let token_last_transaction: Value = data_parsers::get_last_transaction(token_transactions).unwrap();
        let token_current_box_id: String = data_parsers::get_box_id_from_token_data(token_last_transaction);
        let address: String = data_parsers::get_box_address(&token_current_box_id);
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

pub fn reverse_search(address: &str) -> Vec<types::Token> {
    let token_data: Vec<Value> = data_parsers::get_address_tokens(address);
    let token_vector: Vec<types::Token> = data_parsers::convert_to_token_array(token_data);
    let valid_names_vector: Vec<types::Token> = data_parsers::remove_invalid_tokens(token_vector);
    let owned_vector: Vec<types::Token> = data_parsers::check_correct_ownership(valid_names_vector, address);
    return owned_vector;
}

pub fn get_total_amount_owned(address: &str) -> u32 {
    let token_vector: Vec<types::Token> = reverse_search(address);
    let total_amount: u32 = token_vector.len() as u32;
    return total_amount;
}

pub fn get_block_id_registered(name: &str) -> String {
    let token_data: String = data_parsers::create_token_data(&name).unwrap();
    let token_vector: Vec<types::Token> = data_parsers::create_token_vector(token_data);
    let token_id: String = data_parsers::get_asset_minted_at_address(token_vector);
    let first_transaction: Value = endpoints::get_single_transaction_by_token_id(&token_id).unwrap();
    let block_id: String = data_parsers::get_block_id_from_transaction(first_transaction);
    return block_id;
}

pub fn get_block_registered(name: &str) -> String {
    let block_id: String = get_block_id_registered(name);
    let height: String = data_parsers::get_height_from_transaction(&block_id);
    return height;
}

pub fn get_timestamp_registered(name: &str) -> u64 {
    let block_id: String = get_block_id_registered(name);
    let timestamp: String = data_parsers::get_timestamp_from_transaction(&block_id);
    return timestamp.parse::<u64>().unwrap();
}

pub fn get_date_registerd(name: &str) -> String {
    let timestamp: u64 = get_timestamp_registered(name);
    let reformated_time: SystemTime = UNIX_EPOCH + Duration::from_millis(timestamp);
    let datetime: DateTime<Utc> = DateTime::<Utc>::from(reformated_time);
    let timestamp_str: String = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    return timestamp_str;
}