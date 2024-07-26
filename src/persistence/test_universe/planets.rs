use crate::persistence::test_universe::coordinates;
use crate::persistence::{ConstantOrbitSaveData, PlanetSaveData, SaveDataCollection};
use crate::utils::EarthMass;

pub fn create_test_data() -> SaveDataCollection<PlanetSaveData> {
    let mut result = SaveDataCollection::<PlanetSaveData>::default();
    result.add(
        "Planet Alpha".to_string(),
        coordinates::RIGHT,
        EarthMass::from_earth_mass(1, 0),
        ConstantOrbitSaveData {
            radius: 50.0,
            current_rotational_fraction: 0.3,
        },
    );

    result.add(
        "Planet Beta".to_string(),
        coordinates::RIGHT,
        EarthMass::from_earth_mass(1, 50),
        ConstantOrbitSaveData {
            radius: 200.0,
            current_rotational_fraction: 0.7,
        },
    );

    result.add(
        "Planet Gamma".to_string(),
        coordinates::RIGHT,
        EarthMass::from_earth_mass(2, 0),
        ConstantOrbitSaveData {
            radius: 400.0,
            current_rotational_fraction: 0.0,
        },
    );

    result
}
