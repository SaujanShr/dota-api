use std::collections::HashMap;

use gql_client::Client;

use crate::errors::graphql_error::{ Error, ErrorCode };

use crate::models::hero::Hero;

use super::queries::get_heroes_query::{ GET_HEROES_QUERY, Data };

const STRATZ_GQL_ENDPOINT: &str = "https://api.stratz.com/graphql";

pub struct StratzApiClient {
    client: Client
}
impl StratzApiClient {
    pub fn new(token: String) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Authorization", format!("Bearer {}", token));

        let client = Client::new_with_headers(STRATZ_GQL_ENDPOINT, headers);

        Self { client }
    }

    pub async fn get_heroes_query(&self) -> Result<Vec<Hero>, Error> {
        let data = match self.client.query::<Data>(GET_HEROES_QUERY).await {
            Ok(value) => match value {
                Some(data) => Ok(data),
                None => Err(Error { code: ErrorCode::QueryError, message: format!("") })
            },
            Err(e) => Err(Error { code: ErrorCode::HttpError, message: e.to_string() }),
        }?;

        Ok(data.constants.heroes
                    .into_iter()
                    .map(|hero| Hero::new(hero.id, hero.displayName))
                    .collect())
    }
}