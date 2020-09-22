use chrono::NaiveDate;
use juniper::{self, Context, EmptyMutation, RootNode};

use crate::db::DataContext;
use crate::models::*;

impl Context for DataContext {}

pub struct Query;

#[juniper::object(Context = DataContext)]
impl Client {
    fn client_id(&self) -> &str {
        &self.client_id
    }

    fn company_name(&self) -> &str {
        &self.company_name
    }

    fn contact_name(&self) -> &str {
        &self.contact_name
    }

    fn contact_title(&self) -> &str {
        &self.contact_title
    }

    fn phone(&self) -> &str {
        &self.phone
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn invoices(&self, ctx: &DataContext) -> Vec<&Invoice> {
        ctx.invoices
            .iter()
            .filter(|inv| self.client_id == inv.client_id)
            .collect()
    }
}

#[juniper::object(Context = DataContext)]
impl Invoice {    
    fn invoice_id(&self) -> i32 {
        self.invoice_id
    }

    fn invoice_number(&self) -> &str {
        &self.invoice_number
    }

    fn client_id(&self) -> &str {
        &self.client_id
    }

    fn invoice_date(&self) -> NaiveDate {
        self.invoice_date
    }

    fn due_date(&self) -> NaiveDate {
        self.due_date
    }

    fn invoice_items(&self, ctx: &DataContext) -> Vec<&InvoiceItems> {
        ctx.invoice_items
            .iter()
            .filter(|dtl| self.invoice_id == dtl.invoice_id)
            .collect()
    }
}

#[juniper::object(Context = DataContext)]
impl Query {
    fn hello_world() -> &str {
        "Hello, world!"
    }

    fn clients(ctx: &DataContext) -> Vec<&Client> {
        ctx.clients.iter().collect()
    }
}

pub type Mutation = EmptyMutation<DataContext>;

pub type Schema = RootNode<'static, Query, Mutation>;
