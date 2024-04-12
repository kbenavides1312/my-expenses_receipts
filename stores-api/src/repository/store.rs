use postgres::{ Client, NoTls };
// use postgres::Error as PostgresError;

use crate::models::{Store, StoreDBRow};
use crate::constants as constants;
use crate::utils::store as store;

pub fn create(store: Store) -> bool {
    let store_db_row = store::object_to_db_row(store);
    match Client::connect(constants::DB_URL, NoTls) {
        Ok(mut client) => {
            match client.execute("
            INSERT INTO stores (
                nombre,
                nombre_comercial,
                identificacion,
                ubicacion,
                telefono,
                fax,
                correo_electronico
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ",
            &[
                &store_db_row.nombre,
                &store_db_row.nombre_comercial,
                &store_db_row.identificacion,
                &store_db_row.ubicacion,
                &store_db_row.telefono,
                &store_db_row.fax,
                &store_db_row.correo_electronico
            ]) {
                Ok(_) => {
                println!("store inserted");
                return true;
                }
                Err(e) => {
                println!("error inserting store: {:?}", e);
                return false;
                }
            }
        }
        _ => {
            println!("error getting client");
            return false;
        }
    }
}

pub fn get_by_id(store_id: i32) -> Option<Store> {
    match Client::connect(constants::DB_URL, NoTls) {
        Ok(mut client) => 
            match client.query_one("SELECT * FROM stores WHERE id = $1", &[&store_id]) {
                Ok(row) => {
                    let store_db_row = StoreDBRow {
                        id: row.get(0),
                        nombre: row.get(1),
                        nombre_comercial: row.get(2),
                        identificacion: row.get(3),
                        ubicacion: row.get(4),
                        telefono: row.get(5),
                        fax: row.get(6),
                        correo_electronico: row.get(7),
                    };
                    let store = store::db_row_to_object(store_db_row);
                    return Some(store);
                }
                _ =>{
                    println!("No store found with id {}", store_id);
                    return None;
                }
            }
        _ => {
            println!("error getting client");
            return None;
        }
    }
}

//handle get all request
pub fn get_all() -> Vec<Store> {
    match Client::connect(constants::DB_URL, NoTls) {
        Ok(mut client) => {
            let mut store_rows = Vec::new();
            let mut stores = Vec::new();
            for row in client.query("
            SELECT
                id,
                nombre,
                nombre_comercial,
                identificacion,
                ubicacion,
                telefono,
                fax,
                correo_electronico
            FROM stores
            ",
            &[]).unwrap() {
                store_rows.push(StoreDBRow {
                    id: row.get(0),
                    nombre: row.get(1),
                    nombre_comercial: row.get(2),
                    identificacion: row.get(3),
                    ubicacion: row.get(4),
                    telefono: row.get(5),
                    fax: row.get(6),
                    correo_electronico: row.get(7),
                });
            };
            for store in store_rows {
                stores.push( store::db_row_to_object(store) );
            }

            return stores;
        }
        _ => {
            println!("error getting client");
            return Vec::new();
        }
    }
}

pub fn update(store_id: i32, store: Store) -> i32 {
    let store_db_row = store::object_to_db_row(store);
    match Client::connect(constants::DB_URL, NoTls) {
        Ok(mut client) => {
            match client.execute("
            UPDATE stores 
            SET 
                nombre = $2,
                nombre_comercial = $3,
                identificacion = $4,
                ubicacion = $5,
                telefono = $6,
                fax = $7,
                correo_electronico = $8
            WHERE id = $1
            ",
            &[
                &store_id,
                &store_db_row.nombre,
                &store_db_row.nombre_comercial,
                &store_db_row.identificacion,
                &store_db_row.ubicacion,
                &store_db_row.telefono,
                &store_db_row.fax,
                &store_db_row.correo_electronico
            ]) {
                Ok(rows_updated) => {
                    let rows_updated_i32 = rows_updated as i32;
                    println!("stores (id: {}) updated: {}", store_id, rows_updated_i32);
                    return rows_updated_i32;
                }
                Err(e) => {
                    println!("error updating store: {:?}", e);
                    return -1;
                }
            }
        }
        _ => {
            println!("error getting client");
            return -1;
        }
    }
}


pub fn delete(store_id: i32) -> i32 {
    match Client::connect(constants::DB_URL, NoTls) {
        Ok(mut client) => {
            let rows_deleted = client.execute("DELETE FROM stores WHERE id = $1", &[&store_id]).unwrap();
            let rows_deleted_i32 = rows_deleted as i32;
            println!("stores (id: {}) deleted: {}", store_id, rows_deleted_i32);
            return rows_deleted_i32
        }
        _ => return -1
    }
}