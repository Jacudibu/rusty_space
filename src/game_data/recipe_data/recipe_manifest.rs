use crate::game_data::from_mock_data::FromMockData;
use crate::game_data::generic_manifest_without_raw_data::GenericManifestWithoutRawData;
use crate::game_data::{
    RecipeData, RecipeElement, CRYSTAL_ORE_ITEM_ID, HYDROGEN_ITEM_ID, IRON_ORE_ITEM_ID,
    REFINED_METALS_ITEM_ID, REFINED_METALS_RECIPE_ID, SILICA_ITEM_ID, SILICA_RECIPE_ID,
    WAFERS_RECIPE_ID, WAFER_ITEM_ID,
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
            SILICA_RECIPE_ID,
            RecipeData {
                id: SILICA_RECIPE_ID,
                name: "5 CRY -> 10 SIL".into(),
                duration: 10000,
                input: vec![RecipeElement {
                    item_id: CRYSTAL_ORE_ITEM_ID,
                    amount: 5,
                }],
                output: vec![RecipeElement {
                    item_id: SILICA_ITEM_ID,
                    amount: 10,
                }],
            },
        );
        mock_recipes.insert(
            REFINED_METALS_RECIPE_ID,
            RecipeData {
                id: REFINED_METALS_RECIPE_ID,
                name: "5 ORE -> 13 RM".into(),
                duration: 20000,
                input: vec![RecipeElement {
                    item_id: IRON_ORE_ITEM_ID,
                    amount: 5,
                }],
                output: vec![RecipeElement {
                    item_id: REFINED_METALS_ITEM_ID,
                    amount: 13,
                }],
            },
        );
        mock_recipes.insert(
            WAFERS_RECIPE_ID,
            RecipeData {
                id: WAFERS_RECIPE_ID,
                name: "5 SIL + 5 H -> 5 WAF".into(),
                duration: 30000,
                input: vec![
                    RecipeElement {
                        item_id: SILICA_ITEM_ID,
                        amount: 5,
                    },
                    RecipeElement {
                        item_id: HYDROGEN_ITEM_ID,
                        amount: 5,
                    },
                ],
                output: vec![RecipeElement {
                    item_id: WAFER_ITEM_ID,
                    amount: 5,
                }],
            },
        );

        Self::from(mock_recipes)
    }
}
