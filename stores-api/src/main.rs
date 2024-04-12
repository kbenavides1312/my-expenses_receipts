use std::net::{ TcpListener };
use std::env;

extern crate serde_derive;

mod constants;
mod models;
mod utils;
mod repository;
mod controllers;

//main function
fn main() {
    //Set Database
    if let Err(err) = repository::base::set_database() {
        println!("Error setting database: {}", err);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                controllers::base::handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

