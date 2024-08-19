use crate::game_data::{
    Recipe, RecipeElement, RecipeId, DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B, DEBUG_ITEM_ID_C,
    MOCK_RECIPE_A_ID, MOCK_RECIPE_B_ID, MOCK_RECIPE_C_ID,
};
use bevy::prelude::{Asset, Resource, TypePath, World};
use bevy::utils::HashMap;
use leafwing_manifest::identifier::Id;
use leafwing_manifest::manifest::{Manifest, ManifestFormat};
use serde::Deserialize;

#[derive(Resource, Asset, TypePath, Deserialize)]
pub struct RecipeManifest {
    recipes: HashMap<RecipeId, Recipe>,
}

impl RecipeManifest {
    #[must_use]
    pub fn get_by_ref(&self, id: &RecipeId) -> Option<&Recipe> {
        self.recipes.get(id)
    }

    #[must_use]
    pub fn from_mock_data(world: &mut World) -> Self {
        let mut mock_recipes = HashMap::new();
        mock_recipes.insert(
            MOCK_RECIPE_A_ID,
            Recipe {
                id: MOCK_RECIPE_A_ID,
                name: "5C -> 10A".into(),
                duration: 10000,
                input: vec![RecipeElement {
                    item_id: DEBUG_ITEM_ID_C,
                    amount: 5,
                }],
                output: vec![RecipeElement {
                    item_id: DEBUG_ITEM_ID_A,
                    amount: 10,
                }],
            },
        );
        mock_recipes.insert(
            MOCK_RECIPE_B_ID,
            Recipe {
                id: MOCK_RECIPE_B_ID,
                name: "5A -> 13B".into(),
                duration: 20000,
                input: vec![RecipeElement {
                    item_id: DEBUG_ITEM_ID_A,
                    amount: 5,
                }],
                output: vec![RecipeElement {
                    item_id: DEBUG_ITEM_ID_B,
                    amount: 13,
                }],
            },
        );
        mock_recipes.insert(
            MOCK_RECIPE_C_ID,
            Recipe {
                id: MOCK_RECIPE_C_ID,
                name: "5B -> 17C".into(),
                duration: 30000,
                input: vec![RecipeElement {
                    item_id: DEBUG_ITEM_ID_B,
                    amount: 5,
                }],
                output: vec![RecipeElement {
                    item_id: DEBUG_ITEM_ID_C,
                    amount: 17,
                }],
            },
        );

        Self::from_raw_manifest(
            RecipeManifest {
                recipes: mock_recipes,
            },
            world,
        )
        .unwrap()
    }
}

impl Manifest for RecipeManifest {
    type RawManifest = RecipeManifest;
    type RawItem = Recipe;
    type Item = Recipe;
    type ConversionError = std::convert::Infallible;
    const FORMAT: ManifestFormat = ManifestFormat::Custom;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        _world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        Ok(raw_manifest)
    }

    #[must_use]
    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.recipes.get(&id)
    }
}
