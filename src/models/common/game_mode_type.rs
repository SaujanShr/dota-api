use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum GameModeType {
    None,
    AllPick,
    AllPickRanked
}
impl GameModeType {
    pub fn to_gql_str(&self) -> &'static str {
        match self {
            GameModeType::None => "NONE",
            GameModeType::AllPick => "ALL_PICK",
            GameModeType::AllPickRanked => "ALL_PICK_RANKED"
        }
    }
}