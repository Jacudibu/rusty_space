use crate::components::Sector;
use crate::utils::{spawn_helpers, SectorEntity};
use crate::{constants, SpriteHandles};
use bevy::prelude::{
    on_event, App, Commands, Event, EventReader, IntoSystemConfigs, Plugin, Query, Res, Update,
    Vec2,
};

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SectorWasSpawnedEvent>().add_systems(
            Update,
            spawn_asteroids.run_if(on_event::<SectorWasSpawnedEvent>()),
        );
    }
}
#[derive(Event)]
pub struct SectorWasSpawnedEvent {
    pub(crate) sector: SectorEntity,
}

pub fn spawn_asteroids(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut sector_spawns: EventReader<SectorWasSpawnedEvent>,
    mut sectors: Query<&mut Sector>,
) {
    for event in sector_spawns.read() {
        let mut sector = sectors.get_mut(event.sector.into()).unwrap();
        let Some(asteroid_data) = sector.asteroid_data else {
            continue;
        };

        for i in 0..constants::ASTEROID_COUNT {
            let local_pos = Vec2::splat(i as f32 * 25.0);
            spawn_helpers::spawn_asteroid(
                &mut commands,
                &sprites,
                format!("Asteroid {i}"),
                &mut sector,
                event.sector,
                &asteroid_data,
                local_pos,
                0.0,
            );
        }
    }
}
