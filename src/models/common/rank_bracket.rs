use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum RankBracket {
    Uncalibrated,
    Herald,
    Guardian,
    Crusader,
    Archon,
    Legend,
    Ancient,
    Divine,
    Immortal
}
impl RankBracket {
    pub fn to_gql_str(&self) -> &'static str {
        match self {
            RankBracket::Uncalibrated => "UNCALIBRATED",
            RankBracket::Herald => "HERALD",
            RankBracket::Guardian => "GUARDIAN",
            RankBracket::Crusader => "CRUSADER",
            RankBracket::Archon => "ARCHON",
            RankBracket::Legend => "LEGEND",
            RankBracket::Ancient => "ANCIENT",
            RankBracket::Divine => "DIVINE",
            RankBracket::Immortal => "IMMORTAL"
        }
    }
}