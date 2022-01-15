use serde::Deserialize;

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum BuildingDto {
    Extractor(ExtractorDto),
    Processor(ProcessorDto)
}



#[derive(Deserialize,Debug)]
pub struct ExtractorDto {
    #[serde(rename(deserialize = "id"))]
    _id:String,
    #[serde(rename(deserialize = "type"))]
    _kind:String,
    #[serde(rename(deserialize = "power-usage"))]
    _power_usage:i32,
    #[serde(rename(deserialize = "power-extraction"))]
    _normal_extraction_rate:u32
}

#[derive(Deserialize,Debug)]
pub struct ProcessorDto {
    #[serde(rename(deserialize = "id"))]
    _id:String,
    #[serde(rename(deserialize = "type"))]
    _kind:String,
    #[serde(rename(deserialize = "power-usage"))]
    _power_usage:i32

}