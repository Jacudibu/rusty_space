mod recipe;
mod recipe_manifest;

use leafwing_manifest::identifier::Id;

pub use {recipe::*, recipe_manifest::*};

pub type RecipeId = Id<RecipeData>;

const RECIPE_A_STRING: &str = "recipe_a";
const RECIPE_B_STRING: &str = "recipe_b";
const RECIPE_C_STRING: &str = "recipe_c";

pub const MOCK_RECIPE_A_ID: RecipeId = RecipeId::from_name(RECIPE_A_STRING);
pub const MOCK_RECIPE_B_ID: RecipeId = RecipeId::from_name(RECIPE_B_STRING);
pub const MOCK_RECIPE_C_ID: RecipeId = RecipeId::from_name(RECIPE_C_STRING);
