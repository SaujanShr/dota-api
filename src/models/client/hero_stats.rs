use std::collections::HashMap;

use crate::models::{
    common::position_type::PositionType,
    client::position_stats::PositionStats
};

pub struct HeroStats {
    pub position_map: HashMap<PositionType, PositionStats>
}