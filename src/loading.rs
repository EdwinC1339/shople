use serde_derive::{Deserialize, Serialize};
use std::{error::Error, path};

#[derive(Debug, Deserialize)]
pub struct Municipality {
    id: String,
    realname: String,
    lat: f64,
    lon: f64,
}

// Abusing serde a little since these column names are human-friendly
#[derive(Debug, Serialize, Deserialize)]
enum RetailSector {
    #[serde(alias = "Mueblerías")]
    furniture = 1,
    #[serde(alias = "Tiendas de artículos electrónicos")]
    electronics = 2,
    #[serde(alias = "Tiendas de piezas de autos")]
    auto_parts = 3,
    #[serde(alias = "Equipo de patio y jardinería")]
    gardening = 4,
    #[serde(alias = "Tiendas de alimentos especiales")]
    special_food = 5,
    #[serde(alias = "Tiendas de ropa")]
    clothing = 6,
    #[serde(alias = "Tiendas de calzado")]
    footwear = 7,
    #[serde(alias = "Tiendas de joyería, equipaje y artículos de cuero")]
    jewellery = 8,
    #[serde(alias = "Tiendas de deporte, instrumentos musicales y de entretenimiento")]
    sport_hobby = 9,
    #[serde(alias = "Farmacias y droguerías")]
    pharmacy = 10,
    #[serde(alias = "Distribuidores de combustible")]
    fuel = 11,
    #[serde(alias = "Vehículos de motor nuevos y usados")]
    car_dealership = 12,
    #[serde(alias = "Ferreterías y materiales para el hogar")]
    hardware = 13,
    #[serde(alias = "Supermercado y tiendas de bebidas alcohólicas")]
    grocery = 14,
    #[serde(alias = "Tiendas de cosméticos, productos de belleza y perfumes")]
    cosmetics = 15,
    #[serde(alias = "Gasolineras y tiendas de conveniencia")]
    gas_station = 16,
    #[serde(alias = "Tiendas por departamento y otros artículos misceláneos")]
    department = 17,
    #[serde(alias = "Restaurantes y lugares de bebidas alcohólicas")]
    restaurant_bar = 18,
}

#[derive(Debug, Deserialize)]
struct RetailDataRaw {
    id: i32,
    sector: RetailSector,
    revenue_usd: String,
    share: String,
}

#[derive(Debug, Deserialize)]
pub struct RetailData {
    sector: RetailSector,
    revenue_usd: f64,
}

fn process_raw_retail_data(raw: RetailDataRaw) -> RetailData {
    RetailData {
        sector: raw.sector,
        revenue_usd: raw
            .revenue_usd
            .trim_start_matches('$')
            .trim()
            .replace(',', "")
            .parse()
            .unwrap_or(0_f64),
    }
}

pub fn get_municipalities() -> Result<Vec<Municipality>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut municipalities = Vec::<Municipality>::new();
    let rdr = csv::Reader::from_path(path::Path::new("./data/municipalities.csv"));
    for result in rdr?.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let municipality: Municipality = record.deserialize(None)?;
        municipalities.push(municipality);
    }
    Ok(municipalities)
}

fn get_raw_retail_data(id: &str) -> Result<Vec<RetailDataRaw>, Box<dyn Error>> {
    let mut data_rows = Vec::<RetailDataRaw>::new();
    let mut filename = String::from(id);
    filename.push_str(".csv");
    let csv_path = path::Path::new("./data").join(filename);
    let rdr = csv::Reader::from_path(csv_path);

    let mut rdr = rdr?;

    for result in rdr.records() {
        let record = result?;
        let raw_data_row: RetailDataRaw = match record.deserialize(None) {
            Ok(row) => row,
            Err(_) => continue,
        };
        data_rows.push(raw_data_row);
    }
    Ok(data_rows)
}

pub fn get_retail_data(id: &str) -> Result<Vec<RetailData>, Box<dyn Error>> {
    let raw = get_raw_retail_data(id);
    raw.map(|raw_rows| {
        raw_rows
            .into_iter()
            .map(|row| process_raw_retail_data(row))
            .collect()
    })
}
