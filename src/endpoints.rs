use anyhow::{Result};
use serde_json::{Value};

use crate::consts;
use crate::data_parsers;

pub fn get_token_data(token_name: &str, limit: u64, offset: u64) -> Result<Value> {
    let url: String = format!("{}api/v1/tokens/search?query={}&limit={}&offset={}", consts::EXPLORER_API_URL, token_name, limit, offset);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

pub fn get_box_by_id(box_id: &str) -> Result<Value> {
    let url: String = format!("{}api/v1/boxes/{}", consts::EXPLORER_API_URL, box_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

pub fn get_token_transaction_data(token_id: &str) -> Result<Value> {
    let total: u64 = data_parsers::get_max_transactions_for_token(token_id);
    let url: String = format!("{}api/v1/tokens/search?query={}&offset={}", consts::EXPLORER_API_URL, token_id, total-1);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data["items"].to_owned());
}

pub fn get_single_transactions_for_token(token_id: &str) -> Result<Value> {
    let url: String = format!("{}api/v1/tokens/search?query={}&limit=1", consts::EXPLORER_API_URL, token_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}