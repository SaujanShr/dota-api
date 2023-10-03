use gql_client::{ Client, GraphQLError };

use crate::{ 
    errors::graphql_error::{ Error, ErrorCode }, 
    models::api::get_heroes_serde::{ GQLData, GQLHero } 
};

const GET_HEROES_QUERY: &str = "query getHeroes {
    constants {
        heroes {
            id,
            displayName
        }
    }
}";

pub async fn get_heroes(client: &Client) -> Result<Vec<GQLHero>, Error> {
    match client.query::<GQLData>(GET_HEROES_QUERY).await {
        Ok(response) => parse_response(&response),
        Err(error) => parse_error(&error)
    }
}

fn parse_response(response: &Option<GQLData>) -> Result<Vec<GQLHero>, Error> {
    match response {
        Some(data) => Ok(data.constants.heroes.clone()),
        None => Err(Error { code: ErrorCode::MissingDataError, message: format!("The getHeroes query response is valid but empty") })
    }
}

fn parse_error(error: &GraphQLError) -> Result<Vec<GQLHero>, Error> {
    Err(Error { code: ErrorCode::QueryError, message: error.message().to_string() })
}