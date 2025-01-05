mod recipe;
mod recipe_manifest;

use leafwing_manifest::identifier::Id;

use crate::create_id_constants;
pub use {recipe::*, recipe_manifest::*};

pub type RecipeId = Id<RecipeData>;

create_id_constants!(
    RecipeId,
    SILICA_RECIPE,
    REFINED_METALS_RECIPE,
    WAFERS_RECIPE
);
