use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClientModel {
    pub client_id: String,
    pub company_name: String,
    pub contact_name: String,
    pub contact_title: String,
    pub phone: String,
    pub email: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceModel {
    pub invoice_id: i32,
    pub invoice_number: String,
    pub client_id: String,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceItemsModel {
    pub item_id: i32,
    pub invoice_id: i32,
    pub product_id: i32,
    pub description: String,
    pub price: f64,
}
