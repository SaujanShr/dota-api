use std::collections::HashMap;

use gql_client::Client;

use crate::{
    api::queries::{
        get_heroes::get_heroes,
        get_win_week_heroes::get_win_week_positions
    },
    errors::graphql_error::Error,
    models::{
        client::hero::Hero,
        common::{
            game_mode_type::GameModeType,
            rank_bracket::RankBracket,
            region_type::RegionType
        }
    }
};

use super::stratz_api_helper::{get_hero_position_map, gql_heroes_to_heroes};

const STRATZ_GQL_ENDPOINT: &str = "https://api.stratz.com/graphql";

const BRACKETS: &[RankBracket] = &[RankBracket::Archon];
const REGIONS: &[RegionType] = &[RegionType::Europe];
const GAME_MODES: &[GameModeType] = &[GameModeType::AllPickRanked];

pub struct StratzApiClient {
    client: Client
}
impl StratzApiClient {
    pub fn new(token: String) -> Self {
        let headers: HashMap<&str, String> = vec![("Authorization", format!("Bearer {}", token))]
            .into_iter()
            .collect();

        let client = Client::new_with_headers(STRATZ_GQL_ENDPOINT, headers);

        Self { client }
    }

    pub async fn get_heroes(&self) -> Result<Vec<Hero>, Error> {
        let brackets = BRACKETS.to_vec();
        let regions = REGIONS.to_vec();
        let game_modes = GAME_MODES.to_vec();

        let gql_heroes = get_heroes(&self.client).await?;

        let hero_positions = get_win_week_positions(&self.client, &brackets, &regions, &game_modes).await?;

        let hero_position_map = get_hero_position_map(&hero_positions);

        gql_heroes_to_heroes(&gql_heroes, &hero_position_map)
    }
}