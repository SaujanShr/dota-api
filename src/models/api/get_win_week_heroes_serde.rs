use serde::{ Deserialize, Serialize };

use crate::models::{
    common::{
        game_mode_type::GameModeType,
        position_type::PositionType,
        rank_bracket::RankBracket,
        region_type::RegionType
    }, 
    client::position_stats::PositionStats
};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Start {
    pub data: GQLData
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct GQLData {
    pub heroStats: GQLHeroStats
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct GQLHeroStats {
    pub winWeek: Vec<GQLHero>
}

#[allow(non_snake_case)]
#[derive(Clone, Deserialize)]
pub struct GQLHero {
    pub heroId: u16,
    pub winCount: u32,
    pub matchCount: u32
}
impl GQLHero {
    pub fn to_position_stats(&self) -> PositionStats {
        PositionStats { match_count: self.matchCount, win_count: self.winCount }
    }
}

#[derive(Serialize)]
pub struct Vars {
    pub brackets: Vec<&'static str>,
    pub regions: Vec<&'static str>,
    pub game_modes: Vec<&'static str>,
    pub positions: Vec<&'static str>
}
impl Vars {
    pub fn new(brackets: &Vec<RankBracket>, regions: &Vec<RegionType>, game_modes: &Vec<GameModeType>, positions: &Vec<PositionType>) -> Self {
        Vars {
            brackets: brackets
                .iter()
                .map(|bracket| bracket.to_gql_str())
                .collect(),
            regions: regions
                .iter()
                .map(|region| region.to_gql_str())
                .collect(),
            game_modes: game_modes
                .iter()
                .map(|game_mode| game_mode.to_gql_str())
                .collect(),
            positions: positions
                .iter()
                .map(|position| position.to_gql_str())
                .collect(),
        }
    }
}