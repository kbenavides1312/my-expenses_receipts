// use postgres::{ Client, NoTls };
// use postgres::Error as PostgresError;

// use crate::models::Store;
use crate::constants as constants;
use crate::utils as utils;
use crate::repository::store as repository;

//handle post request
pub fn create(request: &str) -> (String, String) {
    match utils::get_store_request_body(&request) {
        Ok(store) => {
            let inserted = repository::create(store);
            if inserted{
                return (constants::OK_RESPONSE.to_string(), "Store created".to_string());
            }
            return (constants::INTERNAL_ERROR.to_string(), "Internal error".to_string());
        }
        err => {
            println!("Error getting store payload: {:?}", err);
            return (constants::INTERNAL_ERROR.to_string(), "Internal error".to_string());
        }
    }
}

// //handle get request
pub fn get_by_id(request: &str) -> (String, String) {
    match utils::get_id(&request).parse::<i32>() {
        Ok(store_id) => {
            let store = repository::get_by_id(store_id);
            if store.is_none(){
                return (constants::NOT_FOUND.to_string(), "Store not found".to_string())
            }
            (constants::OK_RESPONSE.to_string(), serde_json::to_string(&store).unwrap())
        }
        _ => (constants::INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle get all request
pub fn get_all(_request: &str) -> (String, String) {
    let stores = repository::get_all();
    (constants::OK_RESPONSE.to_string(), serde_json::to_string(&stores).unwrap())
}

//handle put request
pub fn update(request: &str) -> (String, String) {

    match (utils::get_id(&request).parse::<i32>(), utils::get_store_request_body(&request)) {
        ( Ok(store_id), Ok(store) ) => {
            let rows_affected = repository::update(store_id, store);
            if rows_affected == -1 {
                return (constants::INTERNAL_ERROR.to_string(), "Internal error".to_string());
            }
            if rows_affected == 0 {
                return (constants::NOT_FOUND.to_string(), "Store not found".to_string());
            }
            (constants::OK_RESPONSE.to_string(), "Store updated".to_string())
        }
        _ => (constants::INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle delete request
pub fn delete(request: &str) -> (String, String) {
    match utils::get_id(&request).parse::<i32>() {
        Ok(store_id) => {
            let rows_affected = repository::delete(store_id);
            if rows_affected == -1 {
                return (constants::INTERNAL_ERROR.to_string(), "Internal error".to_string());
            }
            if rows_affected == 0 {
                return (constants::NOT_FOUND.to_string(), "Store not found".to_string());
            }
            (constants::OK_RESPONSE.to_string(), "Store deleted".to_string())
        }
        _ => (constants::INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}