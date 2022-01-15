use serde::Deserialize;
use crate::model::building::{Building, Extractor, Processor};

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum BuildingDto {
    Extractor(ExtractorDto),
    Processor(ProcessorDto)
}



#[derive(Deserialize,Debug)]
pub struct ExtractorDto {
    #[serde(rename(deserialize = "id"))]
    pub id:String,
    #[serde(rename(deserialize = "type"))]
    pub kind:String,
    #[serde(rename(deserialize = "power-usage"))]
    pub power_usage:i32,
    #[serde(rename(deserialize = "normal-extraction-rate"))]
    pub normal_extraction_rate:u32
}

#[derive(Deserialize,Debug)]
pub struct ProcessorDto {
    #[serde(rename(deserialize = "id"))]
    pub id:String,
    #[serde(rename(deserialize = "type"))]
    pub kind:String,
    #[serde(rename(deserialize = "power-usage"))]
    pub power_usage:i32

}

impl ProcessorDto {
    fn create_processor(&self) -> Processor {
        Processor::new(self.id.clone(), self.kind.clone(), self.power_usage)
    }
}

impl ExtractorDto {
    fn create_extractor(&self) -> Extractor {
        Extractor::new(self.id.clone(), self.kind.clone(), self.power_usage, self.normal_extraction_rate) }
}

impl BuildingDto {
    pub fn create_building(&self) -> Building {
        match self {
            BuildingDto::Extractor(e) => Building::Extractor(e.create_extractor()),
            BuildingDto::Processor(p) => Building::Processor(p.create_processor())
        }
    }
}