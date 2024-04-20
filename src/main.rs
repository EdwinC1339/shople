use serde_derive::Deserialize;
use std::{error::Error, path, process};

#[derive(Debug, Deserialize)]
struct Municipality {
    id: String,
    realname: String,
    lat: f64,
    lon: f64,
}

#[derive(Debug, Deserialize)]
struct RetailDataRaw {
    id: i32,
    category: String,
    revenue_usd: String,
    share: String,
}

#[derive(Debug, Deserialize)]
struct RetailData {
    category: String,
    revenue_usd: f64,
}

fn get_municipalities() -> Result<Vec<Municipality>, Box<dyn Error>> {
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

fn main() {
    let municipalities = get_municipalities().expect("Couldn't load municipalities");
    println!("Loaded {} municipalities...", municipalities.len());
    println!("Head:");
    municipalities
        .into_iter()
        .take(5)
        .for_each(|municipality| println!("{:?}", municipality))
}
