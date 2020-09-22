use std::error::Error;

use csv;

use crate::models::*;

pub struct DataContext {
    pub clients: Vec<Client>,
    pub invoices: Vec<Invoice>,
    pub invoice_items: Vec<InvoiceItems>,
}

impl DataContext {
    pub fn init() -> Result<DataContext, Box<dyn Error>> {
        let clients = read_from_file("./data/Clients.csv")?;
        let invoices = read_from_file("./data/Invoices.csv")?;
        let invoice_items = read_from_file("./data/InvoiceItems.csv")?;

        Ok(DataContext {
            clients,
            invoices,
            invoice_items,
        })
    }
}

fn read_from_file<T>(path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    println!("{}", path);
    let mut reader = csv::Reader::from_path(path)?;

    let mut results = Vec::new();

    for result in reader.deserialize() {
        let record: T = result?;

        results.push(record);
    }

    Ok(results)
}
