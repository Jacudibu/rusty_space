use crate::persistence::test_universe::coordinates::{BOTTOM_LEFT, CENTER, RIGHT, TOP_RIGHT};
use crate::persistence::{
    SaveDataCollection, SectorAsteroidFeatureSaveData, SectorFeatureSaveData, SectorSaveData,
};
use bevy::math::Vec2;

pub fn create_test_data() -> SaveDataCollection<SectorSaveData> {
    let asteroid_data = SectorAsteroidFeatureSaveData::new(Vec2::splat(2.0));

    let mut sectors = SaveDataCollection::<SectorSaveData>::default();
    sectors.add(CENTER);
    sectors.add(RIGHT).with_feature(SectorFeatureSaveData::Star);
    sectors
        .add(TOP_RIGHT)
        .with_feature(SectorFeatureSaveData::AsteroidCloud(asteroid_data));
    sectors.add(BOTTOM_LEFT);

    sectors
}
