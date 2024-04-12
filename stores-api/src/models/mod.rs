use serde::{Serialize, Deserialize};
use std::str;

const NUMBER_DEFAULT: u32 = 0;

fn number_default() -> u32{
    NUMBER_DEFAULT
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub id: Option<i32>,
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Identificacion")]
    pub identificacion: Identity,
    #[serde(rename = "NombreComercial")]
    pub nombre_comercial: String,
    #[serde(rename = "Ubicacion")]
    pub ubicacion: Location,
    #[serde(rename = "Telefono")]
    pub telefono: Phone,
    #[serde(rename = "Fax")]
    pub fax: Phone,
    #[serde(rename = "CorreoElectronico")]
    pub correo_electronico: String,   
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "Provincia")]
    pub provincia: u32,
    #[serde(rename = "Canton")]
    pub canton: u32,
    #[serde(rename = "Distrito")]
    pub distrito: u32,
    #[serde(rename = "OtrasSenas")]
    pub otras_senas: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phone {
    #[serde(rename = "CodigoPais", default = "number_default")]
    pub codigo_pais: u32,
    #[serde(rename = "NumTelefono", default = "number_default")]
    pub num_telefono: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "Tipo")]
    pub tipo: u32,
    #[serde(rename = "Numero")]
    pub numero: u32,
}

pub struct StoreDBRow {
    pub id: Option<i32>,
    pub nombre: String,
    pub identificacion: String,
    pub nombre_comercial: String,
    pub ubicacion: String,
    pub telefono: String,
    pub fax: String,
    pub correo_electronico: String,   
}