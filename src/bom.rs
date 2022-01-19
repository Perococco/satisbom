use std::fmt::{Display, Formatter, Write};
use term::StdoutTerminal;
use crate::bag::{Bag, HashBag};
use crate::colors::{CONSTRUCTOR_COLOR, DEFAULT_COLOR, DURATION_COLOR, RECIPE_NAME_COLOR};
use crate::constants::is_nil;
use crate::model::building::Building;
use crate::model::item::Item;
use crate::model::recipe::Recipe;
use crate::production::Production;

pub struct Bom {
    pub targets:HashBag<Item,f64>,
    pub requirements:HashBag<Item,f64>,
    pub leftovers:HashBag<Item,f64>,
    pub recipes:HashBag<Recipe, f64>,
    pub buildings:HashBag<Building, f64>
}

impl Bom {
    pub(crate) fn create(recipes: HashBag<Recipe, f64>, production: Production<f64>) -> Bom {
        let mut requirements = production.resources().clone();
        requirements += production.available_items();
        requirements -= production.available_left();

        let mut leftovers = production.leftovers().clone();
        leftovers += production.available_left();


        let mut buildings :HashBag<Building,f64> = Default::default();

        let v:Vec<&str> = vec!["s"];
        v.iter();

        recipes.iter()
            .map(|(r, amount)| (r.building(), amount / r.nb_per_minute()))
            .fold(&mut buildings, |bag, (b, amount)| {
                bag.add_item(b.clone(), amount);
                bag
            }
            );

        buildings.clean();

        let targets = production.targets().clone();

        Bom{targets,requirements,leftovers,recipes,buildings}
    }
}

impl Display for Bom  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        for (recipe,amount) in self.recipes.iter() {
            if is_nil(*amount) {
                recipe.format(f,*amount)?;
                f.write_char('\n')?;
            }
        };

        f.write_str("-----\n")?;
        for (building,amount) in self.buildings.iter() {
            f.write_fmt(format_args!("{:<22} {}\n",building.id(),amount))?;
        }

        Ok(())
    }
}


fn display_item_list(term:&mut StdoutTerminal, header:&str,items:&HashBag<Item,f64>) -> crate::error::Result<()> {
    if items.is_empty() {
        return Ok(());
    }

    writeln!(term,"{}",header)?;
    for (item, amount) in items.iter() {
        write!(term,"{:>6} - {item}",amount.to_string())?;
        writeln!(term)?;
    };

    Ok(())
}


impl Bom {


    pub fn display(&self, term:&mut StdoutTerminal) -> crate::error::Result<()> {
        term.fg(DEFAULT_COLOR)?;

        display_item_list(term,"To get:",&self.targets)?;
        display_item_list(term,"You need:",&self.requirements)?;
        display_item_list(term,"Leftovers:",&self.leftovers)?;

        writeln!(term,"=== Recipes ===")?;
        writeln!(term,"  {:>7} - {:<26} {:>3} {:>7} Detail", "#", "Name", "sec" ,"# Cons.")?;
        writeln!(term,"---------------------------------------------------------")?;

        for (recipe, amount) in self.recipes.iter() {
            let nb_need = amount / recipe.nb_per_minute();
            term.fg(DEFAULT_COLOR)?;
            write!(term,"  {:>7.7}",amount.to_string())?;
            write!(term," - ")?;
            term.fg(RECIPE_NAME_COLOR)?;
            write!(term,"{:<26}", recipe.id())?;
            term.fg(DURATION_COLOR)?;
            write!(term," {:>3}", recipe.duration())?;
            term.fg(CONSTRUCTOR_COLOR)?;
            write!(term," {:>7} ", nb_need.to_string())?;

            recipe.display(term, *amount)?;
            writeln!(term)?
        }



        Ok(())
    }



}