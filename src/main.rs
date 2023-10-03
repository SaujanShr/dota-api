use api::stratz_api_client::StratzApiClient;
use utils::secrets_reader::get_token;

pub mod api;
pub mod client;
pub mod errors;
pub mod models;
pub mod utils;

#[tokio::main]
async fn main() {
    let token = get_token().expect("err");
    let client = StratzApiClient::new(token);

    client.get_heroes().await.expect("wtf").iter().for_each(|hero| println!("{}", hero.id));
}
