#[derive(juniper::GraphQLObject)]
pub struct Client {
    pub client_id: String,
    pub company_name: String,
    pub contact_name: String,
    pub contact_title: String,
    pub phone: String,
    pub email: String,
}
