use crate::models::{Store, StoreDBRow, Phone, Location, Identity};
use crate::utils::{parse_u32, parse_string};

pub fn object_to_db_row(store: Store) -> StoreDBRow {
    return StoreDBRow {
        id: None,
        nombre: store.nombre,
        identificacion: format!(
            "{}/{}",
            store.identificacion.tipo,
            store.identificacion.numero
        ),
        nombre_comercial: store.nombre_comercial,
        ubicacion: format!(
            "{}/{}/{}/{}",
            store.ubicacion.provincia,
            store.ubicacion.canton,
            store.ubicacion.distrito,
            store.ubicacion.otras_senas
        ),
        telefono: format!(
            "{}-{}",
            store.telefono.codigo_pais,
            store.telefono.num_telefono
        ),
        fax: format!(
            "{}-{}",
            store.fax.codigo_pais,
            store.fax.num_telefono
        ),
        correo_electronico: store.correo_electronico
    };
}

pub fn db_row_to_object(store: StoreDBRow) -> Store{
    let mut store_identificacion_parts = store.identificacion.split("/");
    let mut store_ubicacion_parts = store.ubicacion.split("/");
    let mut store_telefono_parts = store.telefono.split("-");
    let mut store_fax_parts = store.fax.split("-");
    return Store{
        id: store.id,
        nombre: store.nombre,
        nombre_comercial: store.nombre_comercial,
        identificacion: Identity{
            tipo: parse_u32(store_identificacion_parts.next()),
            numero: parse_u32(store_identificacion_parts.next()),
        },
        ubicacion: Location{
            provincia: parse_u32(store_ubicacion_parts.next()),
            canton: parse_u32(store_ubicacion_parts.next()),
            distrito: parse_u32(store_ubicacion_parts.next()),
            otras_senas: parse_string(store_ubicacion_parts.next())
        },
        telefono: Phone{
            codigo_pais: parse_u32(store_telefono_parts.next()),
            num_telefono: parse_u32(store_telefono_parts.next())
        },
        fax: Phone{
            codigo_pais: parse_u32(store_fax_parts.next()),
            num_telefono: parse_u32(store_fax_parts.next())
        },
        correo_electronico: store.correo_electronico
    };
}
