use std::net::{ TcpStream };
use std::io::{ Read, Write };

use crate::constants as constants;
use crate::controllers::store as store;

//handle requests
pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
            // eprintln!("request: {}", request);

            let (status_line, content) = match &*request {
                
                r if r.starts_with("POST /stores") => store::create(r),
                r if r.starts_with("GET /stores/") => store::get_by_id(r),
                r if r.starts_with("GET /stores") => store::get_all(r),
                r if r.starts_with("PUT /stores/") => store::update(r),
                r if r.starts_with("DELETE /stores/") => store::delete(r),
                _ => (constants::NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}