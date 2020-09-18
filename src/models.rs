use serde::Deserialize;

#[derive(Clone, Deserialize, juniper::GraphQLObject)]
#[serde(rename_all = "PascalCase")]
pub struct Client {
    pub client_id: String,
    pub company_name: String,
    pub contact_name: String,
    pub contact_title: String,
    pub phone: String,
    pub email: String,
}
