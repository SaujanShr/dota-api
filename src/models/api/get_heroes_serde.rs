use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct GQLData {
    pub constants: GQLConstants
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct GQLConstants {
    pub heroes: Vec<GQLHero>
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct GQLHero {
    pub id: u16,
    pub displayName: String
}