use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest_without_raw_data::GenericManifestWithoutRawData;
use crate::game_data::{
    RecipeData, RecipeElement, MOCK_ITEM_A_ID, MOCK_ITEM_B_ID, MOCK_ITEM_C_ID, MOCK_RECIPE_A_ID,
    MOCK_RECIPE_B_ID, MOCK_RECIPE_C_ID,
};
use bevy::prelude::World;
use bevy::utils::HashMap;

/// Contains all parsed crafting recipes.
pub type RecipeManifest = GenericManifestWithoutRawData<RecipeData>;

impl FromMockData for RecipeManifest {
    #[must_use]
    fn from_mock_data(_world: &mut World) -> Self {
        let mut mock_recipes = HashMap::new();
        mock_recipes.insert(
            MOCK_RECIPE_A_ID,
            RecipeData {
                id: MOCK_RECIPE_A_ID,
                name: "5C -> 10A".into(),
                duration: 10000,
                input: vec![RecipeElement {
                    item_id: MOCK_ITEM_C_ID,
                    amount: 5,
                }],
                output: vec![RecipeElement {
                    item_id: MOCK_ITEM_A_ID,
                    amount: 10,
                }],
            },
        );
        mock_recipes.insert(
            MOCK_RECIPE_B_ID,
            RecipeData {
                id: MOCK_RECIPE_B_ID,
                name: "5A -> 13B".into(),
                duration: 20000,
                input: vec![RecipeElement {
                    item_id: MOCK_ITEM_A_ID,
                    amount: 5,
                }],
                output: vec![RecipeElement {
                    item_id: MOCK_ITEM_B_ID,
                    amount: 13,
                }],
            },
        );
        mock_recipes.insert(
            MOCK_RECIPE_C_ID,
            RecipeData {
                id: MOCK_RECIPE_C_ID,
                name: "5B -> 17C".into(),
                duration: 30000,
                input: vec![RecipeElement {
                    item_id: MOCK_ITEM_B_ID,
                    amount: 5,
                }],
                output: vec![RecipeElement {
                    item_id: MOCK_ITEM_C_ID,
                    amount: 17,
                }],
            },
        );

        Self::from(mock_recipes)
    }
}
