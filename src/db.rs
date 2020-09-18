use std::collections::HashMap;
use std::error::Error;

use csv;

use crate::models::*;

pub struct DataContext {
    pub clients: HashMap<usize, Client>,
}

impl DataContext {
    pub fn init() -> Result<DataContext, Box<dyn Error>> {
        let clients = read_from_file("./data/Clients.csv")?;

        Ok(DataContext { clients })
    }
}

fn read_from_file<T>(path: &str) -> Result<HashMap<usize, T>, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let mut reader = csv::Reader::from_path(path)?;

    let mut results = HashMap::new();

    for (idx, result) in reader.deserialize().enumerate() {
        let record: T = result?;

        results.insert(idx, record);
    }

    Ok(results)
}
