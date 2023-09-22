use gql_client::Client;
use serde::{ Deserialize };

use crate::errors::graphql_error::{ Error, ErrorCode };

pub const GET_HEROES_QUERY: &str = "{
    constants {
        heroes {
            id,
            displayName
        }
    }
}";

#[derive(Deserialize)]
pub struct Data {
    pub constants: Constants
}

#[derive(Deserialize)]
pub struct Constants {
    pub heroes: Vec<Hero>
}

#[derive(Deserialize)]
pub struct Hero {
    pub id: u16,
    pub displayName: String
}