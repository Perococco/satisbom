use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::model::building::Extractor;

#[derive(Clone,Hash,Eq)]
pub enum Item {
    Resource(Resource),
    Product(Product),
}

#[derive(Clone,Eq,PartialEq)]
pub struct Product {
    id:String
}

#[derive(Clone,Eq,PartialEq)]
pub struct Resource {
    id:String,
    extractor:Extractor,
    impure:Option<u32>,
    normal:Option<u32>,
    pure:Option<u32>,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (&self,&other) {
            (Item::Resource(r1), Item::Resource(r2)) => r1.id.eq(&r2.id),
            (Item::Product(p1), Item::Product(p2)) => p1.id.eq(&p2.id),
            (_,_) => false
        }
    }
}

impl Hash for Resource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i8(0);
        self.id.hash(state);
    }
}

impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i8(1);
        self.id.hash(state);
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = self.id();
        f.write_str(id)
    }
}


impl Resource {
    pub fn new(id: String, extractor: Extractor, impure: Option<u32>, normal: Option<u32>, pure: Option<u32>) -> Self {
        Resource { id, extractor, impure, normal, pure }
    }
}

impl Product {
    pub fn new(id: String) -> Self {
        Product { id }
    }
}


impl Item {
    pub fn id(&self) -> &str {
        match self {
            Item::Resource(r) => &r.id,
            Item::Product(i) => &i.id
        }
    }
}

