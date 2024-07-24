use crate::persistence::test_universe::coordinates;
use crate::persistence::{ConstantOrbitSaveData, PlanetSaveData, SaveDataCollection};

pub fn create_test_data() -> SaveDataCollection<PlanetSaveData> {
    let mut result = SaveDataCollection::<PlanetSaveData>::default();
    result.add(
        "Planet Alpha".to_string(),
        coordinates::RIGHT,
        100,
        ConstantOrbitSaveData {
            distance: 100.0,
            current_angle: 0.0,
        },
    );

    result
}
