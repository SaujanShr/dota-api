use futures::future::try_join_all;
use gql_client::{Client, GraphQLError};
use strum::IntoEnumIterator;

use crate::{
    models::{
        common::{
            rank_bracket::RankBracket,
            position_type::PositionType,
            game_mode_type::GameModeType,
            region_type::RegionType
        },
        api::get_win_week_heroes_serde::{ GQLData, GQLHero, Vars }
    }, 
    errors::graphql_error::{ Error, ErrorCode }
};

const GET_WIN_WEEK_QUERY_WITH_PARAMS: &str = "query getWinWeek($brackets:[RankBracket], $regions:[BasicRegionType], $gameModes:[GameModeEnumType], $positions:[MatchPlayerPositionType]) {
    heroStats {
        winWeek(bracketIds:$brackets, regionIds:$regions, gameModeIds:$gameModes, positionIds:$positions) {
            heroId,
            winCount,
            matchCount
        }
    }
}";

pub async fn get_win_week_heroes(client: &Client, brackets: &Vec<RankBracket>, regions: &Vec<RegionType>, game_modes: &Vec<GameModeType>, positions: &Vec<PositionType>) -> Result<Vec<GQLHero>, Error> {
    let params = Vars::new(brackets, regions, game_modes, positions);
    
    match client.query_with_vars::<GQLData, Vars>(GET_WIN_WEEK_QUERY_WITH_PARAMS, params).await {
        Ok(response) => parse_response(&response),
        Err(error) => {
            println!("{}", error);
            parse_error(&error)
        }
    }
}

pub async fn get_win_week_positions(client: &Client, brackets: &Vec<RankBracket>, regions: &Vec<RegionType>, game_modes: &Vec<GameModeType>) -> Result<Vec<(PositionType, Vec<GQLHero>)>, Error> {
    try_join_all(
        PositionType::iter().map(|position| async move {
            match get_win_week_heroes(client, brackets, regions, game_modes, &vec![position]).await {
                Ok(heroes) => Ok((position, heroes)),
                Err(err) => Err(err),
            }
        })
    ).await
}

fn parse_response(response: &Option<GQLData>) -> Result<Vec<GQLHero>, Error> {
    match response {
        Some(data) => Ok(data.heroStats.winWeek.clone()),
        None => Err(Error { code: ErrorCode::MissingDataError, message: format!("The getWinWeek query response is valid but empty") })
    }
}

fn parse_error(error: &GraphQLError) -> Result<Vec<GQLHero>, Error> {
    Err(Error { code: ErrorCode::QueryError, message: error.to_string() })
}