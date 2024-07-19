use crate::components::Asteroid;
use crate::entity_selection::Selected;
use crate::utils::AsteroidEntity;
use bevy::color::Alpha;
use bevy::prelude::{Commands, Query, Res, ResMut, Resource, Sprite, Time, Visibility, With};
use bevy::utils::HashSet;

#[derive(Resource, Default)]
pub struct FadingAsteroidsIn {
    pub asteroids: HashSet<AsteroidEntity>,
}

#[derive(Resource, Default)]
pub struct FadingAsteroidsOut {
    pub asteroids: HashSet<AsteroidEntity>,
}

/// Fades asteroid alpha values to 0 before finally turning their visibility off.
/// This will also deselect it once the alpha value reaches 0.
pub fn fade_asteroids_out(
    time: Res<Time>,
    mut commands: Commands,
    mut fading_asteroids: ResMut<FadingAsteroidsOut>,
    mut asteroid_query: Query<(&mut Sprite, &mut Visibility, Option<&Selected>), With<Asteroid>>,
) {
    let mut removals = HashSet::new();

    for entity in &fading_asteroids.asteroids {
        let (mut sprite, mut visibility, selected) = asteroid_query.get_mut(entity.into()).unwrap();

        let new_alpha = sprite.color.alpha() - time.delta_seconds();
        if new_alpha > 0.0 {
            sprite.color.set_alpha(new_alpha);
        } else {
            sprite.color.set_alpha(0.0);
            *visibility = Visibility::Hidden;
            removals.insert(*entity);

            if selected.is_some() {
                commands.entity(entity.into()).remove::<Selected>();
            }
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

        let new_alpha = sprite.color.alpha() + time.delta_seconds();
        if new_alpha < 1.0 {
            sprite.color.set_alpha(new_alpha);
        } else {
            sprite.color.set_alpha(1.0);
            removals.insert(*entity);
        }
    }

    fading_asteroids.asteroids.retain(|x| !removals.contains(x));
}
