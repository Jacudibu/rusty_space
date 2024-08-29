mod recipe;
mod recipe_manifest;

use leafwing_manifest::identifier::Id;

use crate::create_id_constants;
pub use {recipe::*, recipe_manifest::*};

pub type RecipeId = Id<RecipeData>;

create_id_constants!(RecipeId, MOCK_RECIPE_A, MOCK_RECIPE_B, MOCK_RECIPE_C);
