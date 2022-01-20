
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Building {
    Extractor(Extractor),
    Processor(Processor),
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[allow(dead_code)]
pub struct Extractor {
    id: String,
    kind: String,
    power_usage: i32,
    normal_extraction_rate: u32,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[allow(dead_code)]
pub struct Processor {
    id: String,
    kind: String,
    power_usage: i32,
}

impl Building {
    pub fn power_usage(&self) -> i32 {
        match self {
            Building::Extractor(e) => e.power_usage as i32,
            Building::Processor(p) => p.power_usage as i32
        }
    }
}


impl Extractor {
    pub fn new(id: String, kind: String, power_usage: i32, normal_extraction_rate: u32) -> Self {
        Extractor { id, kind, power_usage, normal_extraction_rate }
    }
}

impl Processor {
    pub fn new(id: String, kind: String, power_usage: i32) -> Self {
        Processor { id, kind, power_usage }
    }
}

impl Building {
    pub fn id(&self) -> &str {
        match self {
            Building::Extractor(e) => &e.id,
            Building::Processor(p) => &p.id
        }
    }
}