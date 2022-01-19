use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use term::StdoutTerminal;
use crate::amount::{Amount, AmountF64};
use crate::colors::{DEFAULT_COLOR, DURATION_COLOR};
use crate::model::building::Building;
use crate::model::item::Item;
use crate::model::reactant::Reactant;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Recipe {
    id: String,
    duration: u32,
    building: Building,
    alternate: bool,
    inputs: Vec<Reactant>,
    outputs: Vec<Reactant>,
}

impl Eq for Recipe {}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Recipe {
    pub fn id(&self) -> &str {
        &self.id
    }


    pub fn duration(&self) -> u32 {
        self.duration
    }
}

impl Recipe {
    pub fn new(id: String, duration: u32, building: Building, alternate: bool, inputs: Vec<Reactant>, outputs: Vec<Reactant>) -> Self {
        Recipe { id, duration, building, alternate, inputs, outputs }
    }


}

impl Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let one:AmountF64 = 1.into();
        self.format(f,&one)
    }
}


impl Recipe {
    pub fn display<T>(&self, term:&mut StdoutTerminal, amount:&T) -> crate::error::Result<()> where T:Amount{
        for (i,reactant) in self.inputs.iter().enumerate() {
            if i != 0 {
                term.fg(DEFAULT_COLOR);
                write!(term," + ")?;
            }
            reactant.display(term, amount)?;
        };

        term.fg(DURATION_COLOR);
        write!(term," -> ")?;

        for (i,reactant) in self.outputs.iter().enumerate() {
            if i != 0 {
                term.fg(DURATION_COLOR);
                write!(term," + ")?;
            }
            reactant.display(term, amount)?;
        };
        Ok(())
    }
}

impl Recipe {

    pub fn format<T>(&self, f:&mut Formatter<'_>, amount:&T) -> std::fmt::Result where T:Amount{
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
    pub fn building(&self) -> &Building {
        &self.building
    }

    pub fn alternate(&self) -> bool {
        self.alternate
    }
    pub fn inputs(&self) -> &[Reactant] {
        &self.inputs
    }
    pub fn outputs(&self) -> &[Reactant]{
        &self.outputs
    }
}

impl Recipe {



    //IMPROVE find a find to factorize the three methods below
    /// Retrieve the indices in the referenceBook of the items
    /// involved in this recipe
    pub fn get_involved_items<'a>(&'a self) -> impl Iterator<Item=&Item>  + 'a {
        self.get_input_items().chain(self.get_output_items())
    }


    pub fn get_input_items<'a>(&'a self) -> impl Iterator<Item=&Item>  + 'a {
        self.inputs
            .iter()
            .map(|i| i.item())
    }

    pub fn get_output_items<'a>(&'a self) -> impl Iterator<Item=&Item>  + 'a {
        self.outputs
            .iter()
            .map(|i| i.item())
    }

}