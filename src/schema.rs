use std::collections::HashMap;

use chrono::{Datelike, Duration, Local, NaiveDate};
use juniper::{self, Context, RootNode};

use crate::db::DataContext;

impl Context for DataContext {}

#[derive(Clone, juniper::GraphQLObject)]
/// Information about a client
struct Client {
    /// Client ID
    client_id: String,

    /// Company name
    company_name: String,

    /// Contact name
    contact_name: String,

    /// Contact title
    contact_title: String,

    /// Contact phone number
    phone: String,

    /// Contact email
    email: String,

    /// Current invoices for client
    invoices: Vec<Invoice>,
}

#[derive(Clone, juniper::GraphQLObject)]
/// Information about an invoice
struct Invoice {
    /// Invoice ID
    invoice_id: i32,

    /// Invoice number (INV + {invoice_id})
    invoice_number: String,

    /// Client ID for invoice
    client_id: String,

    /// Date invoice was billed
    invoice_date: NaiveDate,

    /// Date invoice is due
    due_date: NaiveDate,

    /// Invoice items associated with invoice
    invoice_items: Vec<InvoiceItem>,
}

#[derive(Clone, juniper::GraphQLObject)]
/// Information about an invoice item
struct InvoiceItem {
    /// Item ID
    item_id: i32,

    /// ID of associated invoice
    invoice_id: i32,

    /// Product ID
    product_id: i32,

    /// Description of service
    description: String,

    /// Price of service
    price: f64,
}

pub struct Query;

#[juniper::object(Context = DataContext)]
/// An example GraphQL Schema built with Rust
impl Query {
    /// Simple "Hello, world!" query
    fn hello_world() -> &str {
        "Hello, world!"
    }

    /// Client resource to query clients and related invoices
    fn clients(first: Option<i32>, offset: Option<i32>, ctx: &DataContext) -> Vec<Client> {
        let mut client_map = HashMap::new();
        let mut invoice_map = HashMap::new();

        let client_list = ctx.clients.lock().unwrap();

        let (first, offset) = match (first, offset) {
            (Some(f), Some(o)) => (f as usize, o as usize),
            (Some(f), None) => (f as usize, 0),
            (None, Some(o)) => (client_list.len() - o as usize, o as usize),
            (None, None) => (client_list.len(), 0),
        };

        for client_model in client_list.iter().skip(offset).take(first) {
            let client = Client {
                client_id: client_model.client_id.to_owned(),
                company_name: client_model.company_name.to_owned(),
                contact_name: client_model.contact_name.to_owned(),
                contact_title: client_model.contact_title.to_owned(),
                email: client_model.email.to_owned(),
                phone: client_model.phone.to_owned(),
                invoices: Vec::new(),
            };

            client_map.insert(client.client_id.to_owned(), client);
        }

        let invoice_list = ctx.invoices.lock().unwrap();

        for inv_model in invoice_list.iter() {
            if client_map.contains_key(&inv_model.client_id) {
                let invoice = Invoice {
                    invoice_id: inv_model.invoice_id,
                    invoice_number: inv_model.invoice_number.to_owned(),
                    client_id: inv_model.client_id.to_owned(),
                    invoice_date: inv_model.invoice_date,
                    due_date: inv_model.due_date,
                    invoice_items: Vec::new(),
                };

                invoice_map.insert(invoice.invoice_id, invoice);
            }
        }

        let invoice_items_list = ctx.invoice_items.lock().unwrap();

        for inv_item_model in invoice_items_list.iter() {
            if let Some(inv) = invoice_map.get_mut(&inv_item_model.invoice_id) {
                let invoice_item = InvoiceItem {
                    item_id: inv_item_model.item_id,
                    invoice_id: inv_item_model.invoice_id,
                    product_id: inv_item_model.product_id,
                    description: inv_item_model.description.to_owned(),
                    price: inv_item_model.price,
                };

                inv.invoice_items.push(invoice_item);
            }
        }

        for inv in invoice_map.values() {
            if let Some(client) = client_map.get_mut(&inv.client_id) {
                client.invoices.push(inv.clone())
            }
        }

        client_map.values().cloned().collect()
    }

    /// Client resource to get a single client and related invoices
    fn get_client(id: String, ctx: &DataContext) -> Option<Client> {
        let client_list = ctx.clients.lock().unwrap();
        let invoice_list = ctx.invoices.lock().unwrap();
        let invoice_items_list = ctx.invoice_items.lock().unwrap();

        let mut client =
            if let Some(client_model) = client_list.iter().find(|client| client.client_id == id) {
                Client {
                    client_id: client_model.client_id.to_owned(),
                    company_name: client_model.company_name.to_owned(),
                    contact_name: client_model.contact_name.to_owned(),
                    contact_title: client_model.contact_title.to_owned(),
                    email: client_model.email.to_owned(),
                    phone: client_model.phone.to_owned(),
                    invoices: Vec::new(),
                }
            } else {
                return None;
            };

        let mut invoice_map = HashMap::new();

        for inv_model in invoice_list
            .iter()
            .filter(|inv| inv.client_id == client.client_id)
        {
            let invoice = Invoice {
                invoice_id: inv_model.invoice_id,
                invoice_number: inv_model.invoice_number.to_owned(),
                client_id: inv_model.client_id.to_owned(),
                invoice_date: inv_model.invoice_date,
                due_date: inv_model.due_date,
                invoice_items: Vec::new(),
            };

            invoice_map.insert(invoice.invoice_id, invoice);
        }

        for inv_item_model in invoice_items_list.iter() {
            if let Some(inv) = invoice_map.get_mut(&inv_item_model.invoice_id) {
                let invoice_item = InvoiceItem {
                    item_id: inv_item_model.item_id,
                    invoice_id: inv_item_model.invoice_id,
                    product_id: inv_item_model.product_id,
                    description: inv_item_model.description.to_owned(),
                    price: inv_item_model.price,
                };

                inv.invoice_items.push(invoice_item);
            }
        }

        client.invoices = invoice_map.values().cloned().collect();

        Some(client)
    }
}

#[derive(juniper::GraphQLInputObject)]
struct NewClient {
    client_id: String,
    company_name: String,
    contact_name: String,
    contact_title: String,
    phone: String,
    email: String,
}

#[derive(juniper::GraphQLInputObject)]
struct NewInvoiceItem {
    product_id: i32,
    description: String,
    price: f64,
}

pub struct Mutation;

#[juniper::object(Context = DataContext)]
impl Mutation {
    /// Adds a new client to the client list
    fn add_client(new_client: NewClient, ctx: &DataContext) -> Client {
        //  Create GraphQL Client type to return
        let client = Client {
            client_id: new_client.client_id.to_owned(),
            company_name: new_client.company_name.to_owned(),
            contact_name: new_client.contact_name.to_owned(),
            contact_title: new_client.contact_title.to_owned(),
            phone: new_client.phone.to_owned(),
            email: new_client.email.to_owned(),
            invoices: Vec::new(),
        };

        // Insert new client into data store
        ctx.clients
            .lock()
            .unwrap()
            .push(crate::models::ClientModel {
                client_id: new_client.client_id.to_owned(),
                company_name: new_client.company_name.to_owned(),
                contact_name: new_client.contact_name.to_owned(),
                contact_title: new_client.contact_title.to_owned(),
                phone: new_client.phone.to_owned(),
                email: new_client.email.to_owned(),
            });

        client
    }

    /// Adds new invoice for an existing client
    fn add_invoice(client_id: String, ctx: &DataContext) -> Option<Invoice> {
        let client_list = ctx.clients.lock().unwrap();

        if let Some(client) = client_list
            .iter()
            .find(|client| client.client_id == client_id)
        {
            let mut invoice_list = ctx.invoices.lock().unwrap();

            let invoice_id = invoice_list
                .iter()
                .max_by(|a, b| a.invoice_id.cmp(&b.invoice_id))
                .unwrap()
                .invoice_id
                + 1;

            let invoice_number = format!("INV{}", invoice_id);

            let current_month = Local::today().month();
            let current_year = Local::today().year();

            let invoice_date =
                NaiveDate::from_ymd(current_year, current_month + 1, 1) - Duration::days(1);

            let due_date =
                NaiveDate::from_ymd(current_year, current_month + 2, 1) - Duration::days(1);

            let invoice = Invoice {
                invoice_id,
                invoice_number: invoice_number.to_owned(),
                client_id: client_id.to_owned(),
                invoice_date,
                due_date,
                invoice_items: Vec::new(),
            };

            invoice_list.push(crate::models::InvoiceModel {
                invoice_id,
                invoice_number: invoice_number.to_owned(),
                client_id: client_id.to_owned(),
                invoice_date,
                due_date,
            });

            return Some(invoice);
        }

        None
    }

    /// Adds a new invoice item to an existing invoice
    fn add_invoice_item(
        invoice_id: i32,
        new_invoice_item: NewInvoiceItem,
        ctx: &DataContext,
    ) -> Option<InvoiceItem> {
        let mut invoice_list = ctx.invoices.lock().unwrap();

        if let Some(invoice) = invoice_list.iter().find(|inv| inv.invoice_id == invoice_id) {
            let mut invoice_items_list = ctx.invoice_items.lock().unwrap();

            let item_id = invoice_items_list
                .iter()
                .max_by(|a, b| a.item_id.cmp(&b.item_id))
                .unwrap()
                .item_id
                + 1;

            let invoice_item = InvoiceItem {
                item_id,
                invoice_id,
                product_id: new_invoice_item.product_id,
                description: new_invoice_item.description.to_owned(),
                price: new_invoice_item.price,
            };

            invoice_items_list.push(crate::models::InvoiceItemsModel {
                item_id,
                invoice_id,
                product_id: new_invoice_item.product_id,
                description: new_invoice_item.description.to_owned(),
                price: new_invoice_item.price,
            });

            return Some(invoice_item);
        }

        None
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;
