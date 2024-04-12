// use postgres::{ Client, NoTls };
// use postgres::Error as PostgresError;

use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
// use std::env;
use std::any::Any;

use serde_derive::Deserialize;
use serde_xml_rs::from_str;


#[macro_use]
extern crate serde_derive;

//Model: User struct with id, name, email
#[derive(Serialize, Deserialize)]
struct Receipt {
    #[serde(rename = "Clave")]
    clave: i32,
    #[serde(rename = "CodigoActividad")]
    codigo_actividad: i32,
    #[serde(rename = "NumeroConsecutivo")]
    numero_consecutivo: i32,
    #[serde(rename = "FechaEmision")]
    fechaEmision: NaiveDateTime,
    #[serde(rename = "Emisor")]
    emisor: Person,
    #[serde(rename = "Receptor")]
    receptor: Person,
    #[serde(rename = "CondicionVenta")]
    condicion_venta: i32,
    #[serde(rename = "MedioPago")]
    medio_pago: i32,
    #[serde(rename = "DetalleServicio")]
    detalle_servicio: Vec<ItemDetail>,
    #[serde(rename = "ResumenFactura")]
    ResumenFactura: ReceiptSummary,
}

#[derive(Serialize, Deserialize)]
struct Person {
    #[serde(rename = "Nombre")]
    nombre: String,
    #[serde(rename = "Identificacion")]
    identificacion: Identity,
    #[serde(rename = "NombreComercial")]
    nombre_comercial: String,
    #[serde(rename = "Ubicacion")]
    ubicacion: Location,
    #[serde(rename = "Telefono")]
    telefono: Phone,
    #[serde(rename = "Fax")]
    fax: Phone,
    #[serde(rename = "CorreoElectronico")]
    correo_electronico: String,   
}

#[derive(Serialize, Deserialize)]
struct Location {
    #[serde(rename = "Provincia")]
    provincia: i32,
    #[serde(rename = "Canton")]
    canton: i32,
    #[serde(rename = "Distrito")]
    distrito: i32,
    #[serde(rename = "OtrasSenas")]
    otras_senas: String,
}

#[derive(Serialize, Deserialize)]
struct Phone {
    #[serde(rename = "CodigoPais")]
    codigo_pais: i32,
    #[serde(rename = "NumTelefono")]
    num_telefono: i32,
}

#[derive(Serialize, Deserialize)]
struct Identity {
    #[serde(rename = "Tipo")]
    tipo: i32,
    #[serde(rename = "Numero")]
    numero: i32,
}

#[derive(Serialize, Deserialize)]
struct ItemDetail {
    #[serde(rename = "NumeroLinea")]
    NumeroLinea: i32,
    #[serde(rename = "Codigo")]
    Codigo: i32,
    #[serde(rename = "CodigoComercial")]
    CodigoComercial: CommercialCode,
    #[serde(rename = "Cantidad")]
    Cantidad: i32,
    #[serde(rename = "UnidadMedida")]
    UnidadMedida: i32,
    #[serde(rename = "Detalle")]
    Detalle: i32,
    #[serde(rename = "PrecioUnitario")]
    PrecioUnitario: i32,
    #[serde(rename = "MontoTotal")]
    MontoTotal: i32,
    #[serde(rename = "SubTotal")]
    SubTotal: i32,
    #[serde(rename = "BaseImponible")]
    BaseImponible: i32,
    #[serde(rename = "Impuesto")]
    Impuesto: ItemTaxDetail,
    #[serde(rename = "ImpuestoNeto")]
    ImpuestoNeto: i32,
    #[serde(rename = "MontoTotalLinea")]
    MontoTotalLinea: i32,
}

#[derive(Serialize, Deserialize)]
struct ItemTaxDetail {
    #[serde(rename = "Codigo")]
    Codigo: i32,
    #[serde(rename = "CodigoTarifa")]
    CodigoTarifa: i32,
    #[serde(rename = "Tarifa")]
    Tarifa: i32,
    #[serde(rename = "Monto")]
    Monto: i32,
}

#[derive(Serialize, Deserialize)]
struct CommercialCode {
    #[serde(rename = "Tipo")]
    Tipo: i32,
    #[serde(rename = "Codigo")]
    Codigo: i32,
}

#[derive(Serialize, Deserialize)]
struct ReceiptSummary {
    #[serde(rename = "CodigoTipoMoneda")]
    CodigoTipoMoneda: CurrencyCode,
    #[serde(rename = "TotalServGravados")]
    TotalServGravados: i32,
    #[serde(rename = "TotalServExentos")]
    TotalServExentos: i32,
    #[serde(rename = "TotalMercanciasGravadas")]
    TotalMercanciasGravadas: i32,
    #[serde(rename = "TotalMercanciasExentas")]
    TotalMercanciasExentas: i32,
    #[serde(rename = "TotalMercExonerada")]
    TotalMercExonerada: i32,
    #[serde(rename = "TotalGravado")]
    TotalGravado: i32,
    #[serde(rename = "TotalExento")]
    TotalExento: i32,
    #[serde(rename = "TotalExonerado")]
    TotalExonerado: i32,
    #[serde(rename = "TotalVenta")]
    TotalVenta: i32,
    #[serde(rename = "TotalDescuentos")]
    TotalDescuentos: i32,
    #[serde(rename = "TotalVentaNeta")]
    TotalVentaNeta: i32,
    #[serde(rename = "TotalImpuesto")]
    TotalImpuesto: i32,
    #[serde(rename = "TotalIVADevuelto")]
    TotalIVADevuelto: i32,
    #[serde(rename = "TotalOtrosCargos")]
    TotalOtrosCargos: i32,
    #[serde(rename = "TotalComprobante")]
    TotalComprobante: i32,
}

#[derive(Serialize, Deserialize)]
struct CurrencyCode {
    #[serde(rename = "CodigoMoneda")]
    CodigoMoneda: String,
    #[serde(rename = "TipoCambio")]
    TipoCambio: i32,
}
TotalServGravados

    //DATABASE URL
// const DB_URL: &str = env!("DATABASE_URL");

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
// const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

//main function
fn main() {
    //Set Database
    // if let Err(_) = set_database() {
    //     println!("Error setting database");
    //     return;
    // }

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

//handle requests
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 32 * 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Received {} bytes", size);
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
            println!("Received XML:\n{}", request);
            let body = get_user_request_body(&request);
            // let receipt: Receipt = match from_str(&buffer) {
            match from_str::<Receipt>(&body) {
                Ok(parsed_xml) => {
                    // Access the extracted attribute
                    let codigo_actividad = parsed_xml.codigo_actividad;
                    let receptor = parsed_xml.Receptor;

                    // Print the extracted attribute
                    println!("Extracted Codigo Actividad: {}", codigo_actividad);
                    println!("Extracted Receptor: {:?}", Some(receptor));
                    stream.write_all(format!("{}{}", OK_RESPONSE.to_string(), "{}").as_bytes()).unwrap();
                }
                Err(e) => {
                    println!("Error parsing XML: {}", e);
                    stream.write_all(format!("{}{}", INTERNAL_ERROR.to_string(), "{}").as_bytes()).unwrap();
                }
            }


            // request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            // let (status_line, content) = match &*request {
            //     r if r.starts_with("POST /api/users") => handle_post_request(r),
            //     r if r.starts_with("GET /api/users/") => handle_get_request(r),
            //     r if r.starts_with("GET /api/users") => handle_get_all_request(r),
            //     r if r.starts_with("PUT /api/users/") => handle_put_request(r),
            //     r if r.starts_with("DELETE /api/users/") => handle_delete_request(r),
            //     _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            // };
            
            stream.write_all(format!("{}{}", OK_RESPONSE.to_string(), "{}").as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

//handle post request
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

// //db setup
// fn set_database() -> Result<(), PostgresError> {
//     let mut client = Client::connect(DB_URL, NoTls)?;
//     client.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS users (
//             id SERIAL PRIMARY KEY,
//             name VARCHAR NOT NULL,
//             email VARCHAR NOT NULL
//         )
//     "
//     )?;
//     Ok(())
// }

// //Get id from request URL
// fn get_id(request: &str) -> &str {
//     request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
// }

// //deserialize user from request body without id
fn get_user_request_body(request: &str) -> &str {
    request.split("\r\n\r\n").last().unwrap_or_default()
    // serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}