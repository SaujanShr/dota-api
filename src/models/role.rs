use std::fmt;

pub enum Position {
    Position1,
    Position2,
    Position3,
    Position4,
    Position5
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pos = match self {
            Position::Position1 => "Carry",
            Position::Position2 => "Midlane",
            Position::Position3 => "Offlane",
            Position::Position4 => "Soft Support",
            Position::Position5 => "Hard Support"
        };

        write!(f, "{}", pos)
    }
}

pub struct Role {
    pub position: Position,
    pub win_count: u32,
    pub play_count: u32
}