use crate::model::dto::reactant::ReactantDto;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RecipeDto {
    pub id: String,
    pub duration: u32,
    pub building: String,
    pub alternate: bool,
    pub inputs: Vec<ReactantDto>,
    pub outputs: Vec<ReactantDto>,
}


