use serde_json::{Value};

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