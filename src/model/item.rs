use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::model::building::Extractor;

#[derive(Clone,Eq,Debug)]
pub enum Item {
    Resource(Resource),
    Product(Product),
}

#[derive(Clone,Eq,Debug)]
pub struct Product {
    id:String
}

#[derive(Clone,Eq, Debug)]
#[allow(dead_code)]
pub struct Resource {
    id:String,
    extractor:Extractor,
    nodes:Option<Nodes>,
}

impl Item {
    pub fn as_resource(&self) -> Option<&Resource> {
        match self {
            Item::Resource(r) => Some(r),
            Item::Product(_) => None
        }
    }

    pub fn is_resource_picked_manually(&self) -> bool {
        match self {
            Item::Resource(r) => r.extractor.is_manual(),
            Item::Product(_) => false
        }
    }
}

impl Resource {
    pub fn nodes(&self) -> Option<&Nodes> {
        self.nodes.as_ref()
    }
}

#[derive(Clone,Eq, Debug)]
pub struct Nodes {
    impure: u32,
    normal: u32,
    pure: u32,
}

impl Nodes {
    pub fn new(impure: u32, normal: u32, pure: u32) -> Self {
        Nodes { impure, normal, pure }
    }
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

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Item::Resource(r) => r.hash(state),
            Item::Product(p) => p.hash(state)
        }
    }
}

impl Hash for Resource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i8(0);
        self.id.hash(state);
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialEq for Nodes {
    fn eq(&self, other: &Self) -> bool {
        self.impure == other.impure && self.normal == other.normal && self.pure == other.pure
    }
}

impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i8(1);
        self.id.hash(state);
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = self.id();
        f.write_str(id)
    }
}


impl Resource {
    pub fn new(id: String, extractor: Extractor, nodes:Option<Nodes>) -> Self {
        Resource { id, extractor, nodes }
    }

    pub fn max_quantity_per_minute(&self) -> Option<u32> {
        self.nodes().map(|n| n.max_quantity_per_minute())
    }
}

impl Nodes {
    pub fn max_quantity_per_minute(&self) -> u32 {
        self.impure*300 + self.normal*600 + self.pure*780
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

