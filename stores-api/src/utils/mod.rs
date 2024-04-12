pub mod store;

use crate::models::Store;

//Get id from request URL
pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user from request body without id
pub fn get_store_request_body(request: &str) -> Result<Store, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

pub fn parse_u32(value: Option<&str>) -> u32{
    value.unwrap_or("0").parse::<u32>().unwrap_or(0)
}

pub fn parse_string(value: Option<&str>) -> String{
    value.unwrap_or("-").to_string()
}