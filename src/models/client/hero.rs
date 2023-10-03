use crate::models::client::hero_stats::HeroStats;

pub struct Hero {
    pub id: u16,
    pub name: String,
    pub hero_stats: HeroStats
}