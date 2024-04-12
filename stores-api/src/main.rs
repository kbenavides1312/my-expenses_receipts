use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;
use std::error::Error;

use chrono::{TimeZone, Utc};
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use bson::{doc, Document};

use tokio;

// #[macro_use]
// extern crate serde_derive;

// //Model: User struct with id, name, email
// #[derive(Serialize, Deserialize)]
// struct User {
//     id: Option<i32>,
//     name: String,
//     email: String,
// }

//DATABASE URL
const DB_URL: &str = env!("DATABASE_URL");

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

//main function
fn main() {
    //Set Database
    if let Err(err) = set_database() {
        println!("Error setting database: {}", err);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

// //handle requests
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    let mut payload = Document::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            // request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
            // let body = request.split("\r\n\r\n").last().unwrap_or_default()
            let payload = Document::from_reader(&buffer[..size]);
            // let payload = Document::from_reader(&buffer[..size]).as_ref();
            // let payload = Document::from_reader(&mut buffer.as_slice()).unwrap();
            println!("doc: {:?}", payload)

//             let (status_line, content) = match &*request {
//                 r if r.starts_with("POST /api/users") => handle_post_request(r),
//                 r if r.starts_with("GET /api/users/") => handle_get_request(r),
//                 r if r.starts_with("GET /api/users") => handle_get_all_request(r),
//                 r if r.starts_with("PUT /api/users/") => handle_put_request(r),
//                 r if r.starts_with("DELETE /api/users/") => handle_delete_request(r),
//                 _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
//             };

//             stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

// //handle post request
// fn handle_post_request(request: &str) -> (String, String) {
//     match (get_user_request_body(&request), Client::connect(DB_URL, NoTls)) {
//         (Ok(user), Ok(mut client)) => {
//             client
//                 .execute(
//                     "INSERT INTO users (name, email) VALUES ($1, $2)",
//                     &[&user.name, &user.email]
//                 )
//                 .unwrap();

//             (OK_RESPONSE.to_string(), "User created".to_string())
//         }
//         _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
//     }
// }

// //handle get request
// fn handle_get_request(request: &str) -> (String, String) {
//     match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
//         (Ok(id), Ok(mut client)) =>
//             match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
//                 Ok(row) => {
//                     let user = User {
//                         id: row.get(0),
//                         name: row.get(1),
//                         email: row.get(2),
//                     };

//                     (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
//                 }
//                 _ => (NOT_FOUND.to_string(), "User not found".to_string()),
//             }

//         _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
//     }
// }

// //handle get all request
// fn handle_get_all_request(_request: &str) -> (String, String) {
//     match Client::connect(DB_URL, NoTls) {
//         Ok(mut client) => {
//             let mut users = Vec::new();

//             for row in client.query("SELECT id, name, email FROM users", &[]).unwrap() {
//                 users.push(User {
//                     id: row.get(0),
//                     name: row.get(1),
//                     email: row.get(2),
//                 });
//             }

//             (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
//         }
//         _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
//     }
// }

// //handle put request
// fn handle_put_request(request: &str) -> (String, String) {
//     match
//         (
//             get_id(&request).parse::<i32>(),
//             get_user_request_body(&request),
//             Client::connect(DB_URL, NoTls),
//         )
//     {
//         (Ok(id), Ok(user), Ok(mut client)) => {
//             client
//                 .execute(
//                     "UPDATE users SET name = $1, email = $2 WHERE id = $3",
//                     &[&user.name, &user.email, &id]
//                 )
//                 .unwrap();

//             (OK_RESPONSE.to_string(), "User updated".to_string())
//         }
//         _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
//     }
// }

// //handle delete request
// fn handle_delete_request(request: &str) -> (String, String) {
//     match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
//         (Ok(id), Ok(mut client)) => {
//             let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();

//             //if rows affected is 0, user not found
//             if rows_affected == 0 {
//                 return (NOT_FOUND.to_string(), "User not found".to_string());
//             }

//             (OK_RESPONSE.to_string(), "User deleted".to_string())
//         }
//         _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
//     }
// }

//db setup
#[tokio::main]
async fn set_database() -> Result<(), Box<dyn Error>> {
       // Load the MongoDB connection string from an environment variable:
       let options =
          ClientOptions::parse_with_resolver_config(&DB_URL, ResolverConfig::cloudflare())
             .await?;
       let client = Client::with_options(options)?;
    
       // Print the databases in our MongoDB cluster:
       println!("Databases:");
       for name in client.list_database_names(None, None).await? {
          println!("- {}", name);
       }
       Ok(())
}

// //Get id from request URL
// fn get_id(request: &str) -> &str {
//     request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
// }

// //deserialize user from request body without id
// fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
//     serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
// }