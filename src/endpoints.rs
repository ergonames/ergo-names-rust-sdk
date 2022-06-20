use anyhow::{Result};
use serde_json::{Value};

use crate::consts;

pub fn get_token_data(token_name: &str, limit: u64, offset: u64) -> Result<Value> {
    let mut url: String = consts::EXPLORER_API_URL.to_owned();
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

pub fn get_box_by_id(box_id: &str) -> Result<Value> {
    let mut url: String = consts::EXPLORER_API_URL.to_owned();
    url.push_str("api/v1/boxes/");
    url.push_str(box_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}

pub fn get_token_transaction_data(token_id: &str) -> Result<Value> {
    let total: u64 = crate::get_max_transactions_for_token(token_id);
    let mut url: String = consts::EXPLORER_API_URL.to_owned();
    url.push_str("api/v1/assets/search/byTokenId?query=");
    url.push_str(token_id);
    url.push_str("&offset=");
    url.push_str(&(total-1).to_string());
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data["items"].to_owned());
}

pub fn get_single_transactions_for_token(token_id: &str) -> Result<Value> {
    let mut url: String = consts::EXPLORER_API_URL.to_owned();
    url.push_str("api/v1/assets/search/byTokenId?query=");
    url.push_str(token_id);
    url.push_str("&limit=1");
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    return Ok(data);
}