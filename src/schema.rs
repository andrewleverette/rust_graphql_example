use std::collections::HashMap;

use chrono::NaiveDate;
use juniper::{self, Context, EmptyMutation, RootNode};

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
    invoices: Vec<Invoice>
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
    invoice_items: Vec<InvoiceItem>
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
    fn clients(ctx: &DataContext) -> Vec<Client> {
        let mut client_map = HashMap::new();
        let mut invoice_map = HashMap::new();

        for client_model in &ctx.clients {
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
        
        for inv_model in &ctx.invoices {
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

        for inv_item_model in &ctx.invoice_items {
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

    /// Invoice resource to query invoices
    fn invoices(ctx: &DataContext) -> Vec<Invoice> {
        let mut invoice_map = HashMap::new();

        for inv_model in &ctx.invoices {
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

        for inv_item_model in &ctx.invoice_items {
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

        invoice_map.values().cloned().collect()
    }
}

pub type Mutation = EmptyMutation<DataContext>;

pub type Schema = RootNode<'static, Query, Mutation>;
