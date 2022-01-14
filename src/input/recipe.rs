use std::fmt::{Display, Formatter};
use crate::input::reactant::Reactant;
use serde::Deserialize;
use crate::book::Book;
use crate::input::full_book::FullBook;
use crate::error::Result;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Recipe {
    id: String,
    duration: u32,
    building: String,
    alternate: bool,
    inputs: Vec<Reactant>,
    outputs: Vec<Reactant>,
}

impl Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f,1f64)
    }
}

impl Recipe {
    pub fn format(&self, f:&mut Formatter<'_>, amount:f64) -> std::fmt::Result {
        for (i,reactant) in self.inputs.iter().enumerate() {
            if i != 0 {
                f.write_str(" + ")?;
            }
            reactant.format(f, amount)?;
        };

        f.write_str(" -> ")?;

        for (i,reactant) in self.outputs.iter().enumerate() {
            if i != 0 {
                f.write_str(" + ")?;
            }
            reactant.format(f, amount)?
        };
        Ok(())
    }
}


impl Recipe {
    pub fn inputs(&self) -> &Vec<Reactant> {
        &self.inputs
    }
    pub fn outputs(&self) -> &Vec<Reactant> {
        &self.outputs
    }


    pub fn alternate(&self) -> bool {
        self.alternate
    }
}

impl Recipe {



    //IMPROVE find a find to factorize the three methods below
    /// Retrieve the indices in the referenceBook of the items
    /// involved in this recipe
    pub fn get_involved_item_indices<'a>(&'a self, book: &'a FullBook) -> impl Iterator<Item=Result<usize>>  + 'a {
        self.get_input_item_indices(book).chain(self.get_output_item_indices(book))
    }


    pub fn get_input_item_indices<'a>(&'a self, book: &'a FullBook) -> impl Iterator<Item=Result<usize>>  + 'a {
        self.inputs
            .iter()
            .map(|i| i.item_id())
            .map(|id| book.get_item_index(id))
    }

    pub fn get_output_item_indices<'a>(&'a self, book: &'a FullBook) -> impl Iterator<Item=Result<usize>>  + 'a {
        self.outputs
            .iter()
            .map(|i| i.item_id())
            .map(|id| book.get_item_index(id))
    }

}