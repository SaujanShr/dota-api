pub mod api {
    pub mod get_heroes_serde;
    pub mod get_win_week_heroes_serde;
}
pub mod client {
    pub mod hero;
    pub mod hero_stats;
    pub mod position_stats;
}
pub mod common {
    pub mod game_mode_type;
    pub mod position_type;
    pub mod rank_bracket;
    pub mod region_type;
}