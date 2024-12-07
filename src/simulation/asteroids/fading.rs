use crate::components::Asteroid;
use crate::persistence::AsteroidIdMap;
use crate::utils::AsteroidEntity;
use bevy::color::Alpha;
use bevy::prelude::{Commands, Query, Res, ResMut, Resource, Sprite, Time, With};
use bevy::utils::HashSet;

#[derive(Resource, Default)]
pub struct FadingAsteroidsIn {
    pub asteroids: HashSet<AsteroidEntity>,
}

#[derive(Resource, Default)]
pub struct FadingAsteroidsOut {
    pub asteroids: HashSet<AsteroidEntity>,
}

/// Fades asteroid alpha values to 0 before finally despawning them.
pub fn fade_asteroids_out_and_despawn_entities(
    time: Res<Time>,
    mut commands: Commands,
    mut fading_asteroids: ResMut<FadingAsteroidsOut>,
    mut asteroid_query: Query<&mut Sprite, With<Asteroid>>,
    mut asteroid_id_map: ResMut<AsteroidIdMap>,
) {
    let mut removals = HashSet::new();

    for entity in &fading_asteroids.asteroids {
        let mut sprite = asteroid_query.get_mut(entity.into()).unwrap();

        let new_alpha = sprite.color.alpha() - time.delta_secs();
        if new_alpha > 0.0 {
            sprite.color.set_alpha(new_alpha);
        } else {
            removals.insert(*entity);
            asteroid_id_map.remove_by_entity(entity);
            commands.entity(entity.into()).despawn();
        }
    }

    fading_asteroids.asteroids.retain(|x| !removals.contains(x));
}

/// Fades asteroids alpha values to 1
pub fn fade_asteroids_in(
    time: Res<Time>,
    mut fading_asteroids: ResMut<FadingAsteroidsIn>,
    mut asteroid_query: Query<&mut Sprite, With<Asteroid>>,
) {
    let mut removals = HashSet::new();

    for entity in &fading_asteroids.asteroids {
        let mut sprite = asteroid_query.get_mut(entity.into()).unwrap();

        let new_alpha = sprite.color.alpha() + time.delta_secs();
        if new_alpha < 1.0 {
            sprite.color.set_alpha(new_alpha);
        } else {
            sprite.color.set_alpha(1.0);
            removals.insert(*entity);
        }
    }

    fading_asteroids.asteroids.retain(|x| !removals.contains(x));
}
