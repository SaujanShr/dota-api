use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum RegionType {
    China,
    SouthEastAsia,
    NorthAmerica,
    SouthAmerica,
    Europe
}
impl RegionType {
    pub fn to_gql_str(&self) -> &'static str {
        match self {
            RegionType::China => "CHINA",
            RegionType::SouthEastAsia => "SEA",
            RegionType::NorthAmerica => "NORTH_AMERICA",
            RegionType::SouthAmerica => "SOUTH_AMERICA",
            RegionType::Europe => "EUROPE"
        }
    }
}