use std::collections::HashMap;

use crate::{models::{
    common::position_type::PositionType, 
    api::{ get_heroes_serde, get_win_week_heroes_serde },
    client::{hero::Hero, hero_stats::HeroStats}
}, errors::graphql_error::{Error, ErrorCode}};

pub fn get_hero_position_map(positions: &Vec<(PositionType, Vec<get_win_week_heroes_serde::GQLHero>)>) -> HashMap::<u16, Vec<(PositionType, get_win_week_heroes_serde::GQLHero)>> {
    let mut hero_position_map = HashMap::<u16, Vec<(PositionType, get_win_week_heroes_serde::GQLHero)>>::new();

        positions
            .iter()
            .for_each(|(position, heroes)| heroes
                .iter()
                .for_each(|hero| hero_position_map
                    .entry(hero.heroId)
                    .or_default()
                    .push((position.clone(), hero.clone()))
                )
            );

    hero_position_map
}

pub fn gql_heroes_to_heroes(gql_heroes: &Vec<get_heroes_serde::GQLHero>, hero_position_map: &HashMap::<u16, Vec<(PositionType, get_win_week_heroes_serde::GQLHero)>>) -> Result<Vec<Hero>, Error> {
    gql_heroes
        .iter()
        .map(|hero| match hero_position_map.get(&hero.id) {
            Some(hero_position_stats) => Ok(
                Hero {
                id: hero.id,
                name: hero.displayName.clone(),
                hero_stats: HeroStats { position_map: hero_position_stats
                    .iter()
                    .map(|(position, hero)| (position.clone(), hero.to_position_stats()))
                    .collect()
                }
            }
            ),
            None => Err(Error { code: ErrorCode::MissingDataError, message:"Missing data".to_string() }),
        })
        .collect()
}