use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::Recipe;
use crate::error::Error;
use crate::Error::FilterParsingFailed;

#[derive(Clone, serde::Deserialize,serde::Serialize, Debug )]
pub enum RecipeFilter {
    #[serde(rename="not-alternate")]
    NotAlternate,
    #[serde(rename="no-blender")]
    NoBlender,
    #[serde(rename="no-refinery")]
    NoRefinery,
    #[serde(rename="not-manual")]
    NotManual,
    #[serde(rename="not-named")]
    NotNamed(String),
    #[serde(rename="not-using")]
    NotUsing(String),
    #[serde(rename="all-recipes")]
    AllRecipes,
    #[serde(rename="none-of")]
    NoneOf(Vec<RecipeFilter>),
    #[serde(rename="all-of")]
    AllOf(Vec<RecipeFilter>),
    #[serde(rename="any-of")]
    AnyOf(Vec<RecipeFilter>),
    #[serde(rename="not")]
    Not(Box<RecipeFilter>),
}


impl Display for RecipeFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            RecipeFilter::NotAlternate => "not-alternate",
            RecipeFilter::NoBlender => "no-blender",
            RecipeFilter::NoRefinery => "no-refinery",
            RecipeFilter::NotManual => "not-manual",
            RecipeFilter::AllRecipes => "all-recipes",
            RecipeFilter::NoneOf(_) => "none-of()",
            RecipeFilter::AllOf(_) => "all-of",
            RecipeFilter::AnyOf(_) => "any-of",
            RecipeFilter::Not(_) => "not",
            RecipeFilter::NotNamed(_) => "not-named",
            RecipeFilter::NotUsing(_) => "not-using"
        };

        f.write_str(name)
    }
}

impl RecipeFilter {
    pub fn matches(&self, recipe:&Recipe) -> bool {
        match self {
            RecipeFilter::NotAlternate => !recipe.alternate(),
            RecipeFilter::NoRefinery => !recipe.uses_a_refinery(),
            RecipeFilter::NotManual => !recipe.uses_manual_resources(),
            RecipeFilter::NotNamed(name) => !name.eq_ignore_ascii_case(recipe.id()),
            RecipeFilter::AllRecipes => true,
            RecipeFilter::NoneOf(filters) => filters.iter().all(|f| !f.matches(recipe)),
            RecipeFilter::AllOf(filters) => filters.iter().all(|f| f.matches(recipe)),
            RecipeFilter::AnyOf(filters) => filters.iter().any(|f| f.matches(recipe)),
            RecipeFilter::Not(filter) => !filter.matches(recipe),
            RecipeFilter::NoBlender => !recipe.uses_a_blender(),
            RecipeFilter::NotUsing(item_id) => !recipe.uses_item(item_id)
        }
    }
}

impl FromStr for RecipeFilter {
    type Err = Error;

    fn from_str(f: &str) -> Result<Self, Self::Err> {
        match f {
            "not-alternate" => Ok(RecipeFilter::NotAlternate),
            "not-manual" => Ok(RecipeFilter::NotManual),
            "no-refinery" => Ok(RecipeFilter::NoRefinery),
            "no-blender" => Ok(RecipeFilter::NoBlender),
            "all-recipes" => Ok(RecipeFilter::AllRecipes),
            filter => {
                if let Some(recipe_name) = filter.strip_prefix("wo_") {
                    Ok(RecipeFilter::NotNamed(recipe_name.to_string()))
                } else if let Some(item_id) = filter.strip_prefix("nu_") {
                    Ok(RecipeFilter::NotUsing(item_id.to_string()))
                } else {
                    Err(FilterParsingFailed(filter.to_string()))
                }
            }
        }
    }
}