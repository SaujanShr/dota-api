use serde::Serialize;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, strum_macros::EnumIter)]
pub enum PositionType {
    Position1,
    Position2,
    Position3,
    Position4,
    Position5 
}
impl PositionType {
    pub fn to_gql_str(&self) -> &'static str {
        match self {
            PositionType::Position1 => "POSITION_1",
            PositionType::Position2 => "POSITION_2",
            PositionType::Position3 => "POSITION_3",
            PositionType::Position4 => "POSITION_4",
            PositionType::Position5 => "POSITION_5"
        }
    }
}