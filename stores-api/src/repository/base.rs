use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;

use crate::constants as constants;

//db setup
pub fn set_database() -> Result<(), PostgresError> {
   let mut client = Client::connect(constants::DB_URL, NoTls)?;
   client.batch_execute(
       "
DROP TABLE stores;
-- DROP TYPE Identity;
-- DROP TYPE Phone;
-- DROP TYPE Location;

/*
CREATE TYPE Identity as (
    tipo OID,
    numero OID
);

CREATE TYPE Phone as (
    codigo_pais OID,
    num_telefono OID
);

CREATE TYPE Location as (
    provincia OID,
    canton OID,
    distrito OID,
    otras_senas VARCHAR
);
*/

CREATE TABLE IF NOT EXISTS stores (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    nombre_comercial VARCHAR,
    identificacion VARCHAR NOT NULL,
    ubicacion VARCHAR,
    telefono VARCHAR,
    fax VARCHAR,
    correo_electronico VARCHAR
);
   "
   )?;
   Ok(())
}