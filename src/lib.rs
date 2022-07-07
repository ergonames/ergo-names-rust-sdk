use anyhow::{Result};
use serde_json::{Value};
use chrono::prelude::*;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Debug, Clone)]
pub struct  Token {
    pub name: String,
    pub id: String,
    pub box_id: String,
}

pub const EXPLORER_API_URL: &str = "https://api-testnet.ergoplatform.com/";
pub const MINT_ADDRESS: &str = "3WycHxEz8ExeEWpUBwvu1FKrpY8YQCiH1S9PfnAvBX1K73BXBXZa";

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

pub fn check_name_price(name: &str) -> i32 {
    let _: String = reformat_name(name);
    return 0;
}

pub fn resolve_ergoname(name: &str, explorer_url: Option<String>) -> Option<String> {
    let token_data: String = create_token_data(&name, explorer_url.clone()).unwrap();
    if token_data != "None" {
        let token_vector: Vec<Token> = create_token_vector(token_data);
        let token_id: String = get_asset_minted_at_address(token_vector);
        let token_transactions: Value = get_token_transaction_data(&token_id, explorer_url).unwrap();
        let token_last_transaction: Value = get_last_transaction_for_token(token_transactions);
        let token_current_box_id: String = get_box_id_from_token_data(token_last_transaction);
        let address: String = get_box_address(&token_current_box_id);
        return Some(address);
    }
    return None
}

pub fn check_already_registered(name: &str, explorer_url: Option<String>) -> bool {
    let address: Option<String> = resolve_ergoname(name, explorer_url);
    if address.is_none() {
        return false;
    } else {
        return true;
    }
}

pub fn reverse_search(address: &str, explorer_url: Option<String>) -> Option<Vec<Token>> {
    let token_data: Vec<Value> = get_address_tokens(address, explorer_url);
    if token_data.len() != 0 {
        let token_vector: Vec<Token> = convert_to_token_array(token_data);
        let valid_names_vector: Vec<Token> = remove_invalid_tokens(token_vector);
        let owned_vector: Vec<Token> = check_correct_ownership(valid_names_vector, address);
        return Some(owned_vector);
    }
    return None;
}

pub fn get_total_amount_owned(address: &str, explorer_url: Option<String>) -> Option<u32> {
    let token_vector: Option<Vec<Token>> = reverse_search(address, explorer_url);
    if token_vector.is_some() {
        let total_amount: u32 = token_vector.unwrap().len() as u32;
        return Some(total_amount);
    }
    return None;
}

pub fn get_block_id_registered(name: &str, explorer_url: Option<String>) -> Option<String> {
    let token_data: String = create_token_data(&name, explorer_url).unwrap();
    if token_data != "None" {
        let token_vector: Vec<Token> = create_token_vector(token_data);
        let token_id: String = get_asset_minted_at_address(token_vector);
        let first_transaction: Value = get_single_transaction_by_token_id(&token_id, None).unwrap();
        let block_id: String = get_block_id_from_transaction(first_transaction);
        return Some(block_id);
    }
    return None;
}

pub fn get_block_registered(name: &str, explorer_url: Option<String>) -> Option<i32> {
    let block_id: Option<String> = get_block_id_registered(name, explorer_url);
    if block_id.is_some() {
        let height_str: String = remove_quotes(get_height_from_transaction(&block_id.unwrap()));
        let height: i32 = height_str.parse::<i32>().unwrap();
        return Some(height);
    }
    return None;
}

pub fn get_timestamp_registered(name: &str, explorer_url: Option<String>) -> Option<u64> {
    let block_id: Option<String> = get_block_id_registered(name, explorer_url);
    if block_id.is_some() {
        let timestamp: String = get_timestamp_from_transaction(&block_id.unwrap());
        return Some(timestamp.parse::<u64>().unwrap());
    }
    return None;
}

pub fn get_date_registerd(name: &str, explorer_url: Option<String>) -> Option<String> {
    let timestamp: Option<u64> = get_timestamp_registered(name, explorer_url);
    if timestamp.is_some() {
        let reformated_time: SystemTime = UNIX_EPOCH + Duration::from_millis(timestamp.unwrap());
        let datetime: DateTime<Utc> = DateTime::<Utc>::from(reformated_time);
        let timestamp_str: String = datetime.format("%Y-%m-%d").to_string();
        return Some(timestamp_str);
    }
    return None;
}

fn remove_quotes(i_str: String) -> String {
    let n_str: String = i_str.replace('"', "");
    return n_str;
}

fn get_token_data(token_name: &str, limit: u64, offset: u64, explorer_url: Option<String>) -> Result<Value> {
    if explorer_url.is_none() {
        let url: String = format!("{}api/v1/tokens/search?query={}&limit={}&offset={}", EXPLORER_API_URL, token_name, limit, offset);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    } else {
        let url: String = format!("{}api/v1/tokens/search?query={}&limit={}&offset={}", explorer_url.unwrap(), token_name, limit, offset);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    }
}

fn get_box_by_id(box_id: &str, explorer_url: Option<String>) -> Result<Value> {
    if explorer_url.is_none() {
        let url: String = format!("{}api/v1/boxes/{}", EXPLORER_API_URL, box_id);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    } else {
        let url: String = format!("{}api/v1/boxes/{}", explorer_url.unwrap(), box_id);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    }
}

fn get_block_by_id(block_id: &str, explorer_url: Option<String>) -> Result<Value> {
    if explorer_url.is_none() {
        let url: String = format!("{}api/v1/blocks/{}", EXPLORER_API_URL, block_id);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    } else {
        let url: String = format!("{}api/v1/blocks/{}", explorer_url.unwrap(), block_id);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    }
}

fn get_token_transaction_data(token_id: &str, explorer_url: Option<String>) -> Result<Value> {
    if explorer_url.is_none() {
        let url: String = format!("{}api/v1/assets/search/byTokenId?query={}", EXPLORER_API_URL, token_id);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data["items"].to_owned());
    } else {
        let url: String = format!("{}api/v1/assets/search/byTokenId?query={}", explorer_url.unwrap(), token_id);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data["items"].to_owned());
    }
}

fn get_single_transaction_by_token_id(token_id: &str, explorer_url: Option<String>) -> Result<Value> {
    if explorer_url.is_none() {
        let url: String = format!("{}api/v1/assets/search/byTokenId?query={}&limit=1", EXPLORER_API_URL, token_id);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    } else {
        let url: String = format!("{}api/v1/assets/search/byTokenId?query={}&limit=1", explorer_url.unwrap(), token_id);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    }
}

fn get_address_confirmed_balance(address: &str, explorer_url: Option<String>) -> Result<Value> {
    if explorer_url.is_none() {
        let url: String = format!("{}api/v1/addresses/{}/balance/confirmed", EXPLORER_API_URL, address);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    } else {
        let url: String = format!("{}api/v1/addresses/{}/balance/confirmed", explorer_url.unwrap(), address);
        let resp: String = reqwest::blocking::get(url)?.text()?;
        let data: Value = serde_json::from_str(&resp)?;
        return Ok(data);
    }
}

fn create_token_data(token_name: &str, explorer_url: Option<String>) -> Result<String> {
    let total: u64 = get_token_data(&token_name, 1, 0, explorer_url.clone()).unwrap()["total"].to_owned().as_u64().unwrap();
    let needed_calls: u64 = (total / 500) + 1;
    let mut offset: u64 = 0;
    let mut transaction_data: String = "".to_owned();
    if total > 0 {
        for _i in 0..needed_calls {
            transaction_data = transaction_data + &get_token_data(&token_name, 500, offset, explorer_url.clone()).unwrap()["items"].to_string();
            offset = offset + 500;
        }
        return Ok(transaction_data);
    } else {
        return Ok("None".to_string());
    }
}

fn create_token_vector(data: String) -> Vec<Token> {
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

fn get_asset_minted_at_address(token_vector: Vec<Token>) -> String{
    for i in token_vector {
        let address: String = get_box_address(&i.box_id);
        if address == MINT_ADDRESS.to_owned() {
            return i.id;
        }
    }
    return "None".to_owned();
}

fn get_box_address(box_id: &str) -> String {
    let box_data: Value = get_box_by_id(box_id, None).unwrap();
    let address: String = remove_quotes(box_data["address"].to_string());
    return address;
}

fn get_last_transaction_for_token(data: Value) -> Value {
    let length: usize = data.as_array().unwrap().len();
    let last_borrowed: &Value = &data.get(length-1).unwrap();
    let last: Value = last_borrowed.to_owned();
    return last;
}

fn get_box_id_from_token_data(data: Value) -> String{
    let box_id: String = data["boxId"].to_string();
    return remove_quotes(box_id);
}

fn get_address_tokens(address: &str, explorer_url: Option<String>) -> Vec<Value> {
    let balance: Value = get_address_confirmed_balance(address, explorer_url).unwrap();
    let tokens: &Vec<Value> = &balance["tokens"].as_array().unwrap().to_owned();
    return tokens.to_owned();
}

fn convert_to_token_array(data: Vec<Value>) -> Vec<Token> {
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

fn remove_invalid_tokens(token_vector: Vec<Token>) -> Vec<Token> {
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

fn check_correct_ownership(token_vector: Vec<Token>, user_address: &str) -> Vec<Token> {
    let mut new_token_vector: Vec<Token> = Vec::new();
    for i in 0..token_vector.len() {
        let token_address = get_box_address(&token_vector.get(i).unwrap().box_id);
        if token_address == user_address {
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
    let block_data: Value = get_block_by_id(block_id, None).unwrap();
    let height: String = block_data["block"]["header"]["height"].to_string();
    return remove_quotes(height);
}

fn get_timestamp_from_transaction(block_id: &str) -> String {
    let block_data: Value = get_block_by_id(block_id, None).unwrap();
    let timestamp: String = block_data["block"]["header"]["timestamp"].to_string();
    return remove_quotes(timestamp);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const NAME: &str = "~balb";
    const NULL_NAME: &str = "~zack";
    const ADDRESS: &str = "3WwKzFjZGrtKAV7qSCoJsZK9iJhLLrUa3uwd4yw52bVtDVv6j5TL";
    const NULL_ADDRESS: &str = "3Wxf2LxF8HUSzfnT6bDGGUDNp1YMvWo5JWxjeSpszuV6w6UJGLSf";

    #[test]
    fn test_resolve_ergoname() {
        assert_eq!(resolve_ergoname(NAME, None).unwrap(), "3WwKzFjZGrtKAV7qSCoJsZK9iJhLLrUa3uwd4yw52bVtDVv6j5TL");
    }

    #[test]
    fn test_null_resolve_ergoname() {
        assert_eq!(resolve_ergoname(NULL_NAME, None), None);
    }

    #[test]
    fn test_check_already_registered() {
        assert_eq!(check_already_registered(NAME, None), true);
    }

    #[test]
    fn test_null_check_already_registered() {
        assert_eq!(check_already_registered(NULL_NAME, None), false);
    }

    #[test]
    fn test_check_name_valid() {
        assert_eq!(check_name_valid(NAME), true);
    }

    #[test]
    fn test_null_check_name_valid() {
        assert_eq!(check_name_valid(NULL_NAME), true);
    }

    #[test]
    fn test_check_name_price() {
        assert_eq!(check_name_price(NAME), 0);
    }

    #[test]
    fn test_null_check_name_price() {
        assert_eq!(check_name_price(NULL_NAME), 0);
    }

    #[test]
    fn test_get_block_id_registered() {
        assert_eq!(get_block_id_registered(NAME, None).unwrap(), "a5e0ab7f95142ceee7f3b6b5a5318153b345292e9aaae7c56825da115e196d08");
    }

    #[test]
    fn test_null_get_block_id_registered() {
        assert_eq!(get_block_id_registered(NULL_NAME, None), None);
    }

    #[test]
    fn test_get_block_registered() {
        assert_eq!(get_block_registered(NAME, None).unwrap(), 60761);
    }

    #[test]
    fn test_null_get_block_registered() {
        assert_eq!(get_block_registered(NULL_NAME, None), None);
    }

    #[test]
    fn test_get_timestamp_registered() {
        assert_eq!(get_timestamp_registered(NAME, None).unwrap(), 1656968987794);
    }

    #[test]
    fn test_null_get_timestamp_registered() {
        assert_eq!(get_timestamp_registered(NULL_NAME, None), None);
    }

    #[test]
    fn test_get_date_registered() {
        assert_eq!(get_date_registerd(NAME, None).unwrap(), "2022-07-04");
    }

    #[test]
    fn test_null_get_date_registered() {
        assert_eq!(get_date_registerd(NULL_NAME, None), None);
    }

    #[test]
    fn test_get_total_amount_owned() {
        assert_eq!(get_total_amount_owned(ADDRESS, None).unwrap(), 1);
    }

    #[test]
    fn test_null_get_total_amount_owned() {
        assert_eq!(get_total_amount_owned(NULL_ADDRESS, None), None);
    }

    #[test]
    fn test_reverse_search() {
        let legit_token = Token {
            name: String::from("~balb"),
            id: String::from("2b41b93d22a46de0b0ed9c8b814b766298adbf2ff304f83ee2426f47ac33d9b8"),
            box_id: String::from("82b9b9773471041f1fa4763dc14e156f6c044e41d99ac7ef34709be4fef7c6d6"),
        };
        let mut vec = Vec::<Token>::new();
        vec.push(legit_token);
        assert_eq!(vec_compare(reverse_search(ADDRESS, None), Some(vec)), true);
    }

    #[test]
    fn test_null_reverse_search() {
        assert_eq!(vec_compare(reverse_search(NULL_ADDRESS, None), None), true);
    }

    fn vec_compare(va: Option<Vec<Token>>, vb: Option<Vec<Token>>) -> bool {
        if va.is_none() && va.is_some() {
            return false;
        }
        if vb.is_none() && vb.is_none() {
            return true;
        }
        let vau: Vec<Token> = va.unwrap();
        let vbu: Vec<Token> = vb.unwrap();
        if vau.len() != vbu.len() {
            return false;
        }
        for i in 0..vau.len() {
            if vau.get(i).unwrap().name != vbu.get(i).unwrap().name {
                return false;
            }
        }
        return true;
    }

}