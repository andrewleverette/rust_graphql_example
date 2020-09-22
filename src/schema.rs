use std::collections::HashMap;

use juniper::{self, Context, EmptyMutation, RootNode};

use crate::db::DataContext;
use crate::models;

impl Context for DataContext {}

#[derive(juniper::GraphQLObject)]
struct Client {
    client: models::ClientModel,
    invoices: Vec<Invoice>
}

#[derive(Clone, juniper::GraphQLObject)]
struct Invoice {
    invoice: models::InvoiceModel,
    invoice_items: Vec<models::InvoiceItemsModel>
}

pub struct Query;

#[juniper::object(Context = DataContext)]
impl Query {
    fn hello_world() -> &str {
        "Hello, world!"
    }

    fn clients(ctx: &DataContext) -> Vec<Client> {
        let mut invoice_item_map: HashMap<i32, Vec<models::InvoiceItemsModel>> = HashMap::new();

        for item in &ctx.invoice_items {
            match invoice_item_map.get_mut(&item.invoice_id) {
                Some(items) => items.push(item.clone()),
                None => {
                    invoice_item_map.insert(item.invoice_id, vec![item.clone()]);
                }
            }
        }

        let mut invoice_map: HashMap<String, Vec<Invoice>> = HashMap::new();

        for inv in &ctx.invoices {
            let invoice_items = match invoice_item_map.get(&inv.invoice_id) {
                Some(items) => items.clone(),
                None => Vec::new()
            };

            let invoice = Invoice {
                invoice: inv.clone(),
                invoice_items
            };

            match invoice_map.get_mut(&inv.client_id) {
                Some(invoices) => invoices.push(invoice),
                None => {
                    invoice_map.insert(inv.client_id.to_owned(), vec![invoice]);
                }
            }
        }

        let mut clients = Vec::new();

        for client in &ctx.clients {
            let invoices = match invoice_map.get(&client.client_id) {
                Some(invoices) => invoices.clone(),
                None => Vec::new()
            };

            let client = Client {
                client: client.clone(),
                invoices
            };

            clients.push(client);
        }

        clients
    }
}

pub type Mutation = EmptyMutation<DataContext>;

pub type Schema = RootNode<'static, Query, Mutation>;
