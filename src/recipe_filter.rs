use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::error::Error;
use crate::Error::FilterParsingFailed;
use crate::Recipe;

#[derive(Clone, serde::Deserialize,serde::Serialize, Debug )]
pub enum RecipeFilter {
    #[serde(rename="not-alternate")]
    NotAlternate,
    #[serde(rename="no-blender")]
    NoBlender,
    #[serde(rename="not-manual")]
    NotManual,
    #[serde(rename="not-named")]
    NotNamed(HashSet<String>),
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
            RecipeFilter::NotManual => "not-manual",
            RecipeFilter::AllRecipes => "all-recipes",
            RecipeFilter::NoneOf(_) => "none-of()",
            RecipeFilter::AllOf(_) => "all-of",
            RecipeFilter::AnyOf(_) => "any-of",
            RecipeFilter::Not(_) => "not",
            RecipeFilter::NotNamed(_) => "not-named"
        };

        f.write_str(name)
    }
}

impl RecipeFilter {
    pub fn matches(&self, recipe:&Recipe) -> bool {
        match self {
            RecipeFilter::NotAlternate => !recipe.alternate(),
            RecipeFilter::NotManual => !recipe.uses_manual_resources(),
            RecipeFilter::NotNamed(names) => !names.contains(recipe.id()),
            RecipeFilter::AllRecipes => true,
            RecipeFilter::NoneOf(filters) => filters.iter().all(|f| !f.matches(recipe)),
            RecipeFilter::AllOf(filters) => filters.iter().all(|f| f.matches(recipe)),
            RecipeFilter::AnyOf(filters) => filters.iter().any(|f| f.matches(recipe)),
            RecipeFilter::Not(filter) => !filter.matches(recipe),
            RecipeFilter::NoBlender => !recipe.uses_a_blender()
        }
    }
}

impl FromStr for RecipeFilter {
    type Err = Error;

    fn from_str(f: &str) -> Result<Self, Self::Err> {
        match f {
            "no-alternate" => Ok(RecipeFilter::NotAlternate),
            "not-manual" => Ok(RecipeFilter::NotManual),
            "no-blender" => Ok(RecipeFilter::NoBlender),
            _ => Err(FilterParsingFailed(f.to_string()))
        }
    }
}