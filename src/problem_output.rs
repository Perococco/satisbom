use std::fmt::{Display, Formatter, Write};
use crate::book::Book;
use crate::input::recipe::Recipe;

pub struct ProblemOutput<'a> {
    pub book:&'a dyn Book,
    pub used_recipes:Vec<(&'a Recipe,f64)>
}


impl Display for ProblemOutput<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (recipe,amount) in &self.used_recipes {
            if *amount < 1e-6 {
                continue
            }
            recipe.format(f,*amount)?;
            f.write_char('\n')?;
        };

        Ok(())
    }
}