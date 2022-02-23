use anyhow::{anyhow, Result};

fn create_url(name: &str) -> String {
    let mut url = "https://testnet-api.ergonames.com".to_owned();
    url += "/ergonames/resolve/";
    url += name;
    return url;
}

fn make_request(url: String) -> Result<String> {
    let resp = reqwest::blocking::Client::new().get(url).send()?;
    let resp_json = json::parse(&resp.text()?)?;
    if let Some(addr) = resp_json["ergo"].as_str() {
        return Ok(addr.to_owned());
    } else {
        Err(anyhow!("Failed to parse json"))
    }
}

fn result_to_option(response: Result<String>) -> Option<String> {
    return response.ok();
}

fn option_to_string(opt: Option<String>) -> String {
    if opt.is_none() {
        return "None".to_owned();
    } else {
        return opt.unwrap();
    }
}

pub fn get_owner_address(name: &str) -> String {
    let url = create_url(name);
    let response = make_request(url);
    let option = result_to_option(response);
    let address = option_to_string(option);
    return address;
}

pub fn check_address_exists(name: &str) -> bool {
    let address = get_owner_address(name);
    if address == "None" {
        return false;
    } else {
        return true;
    }
}