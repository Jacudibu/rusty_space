use crate::SpriteHandles;
use crate::persistence::{CelestialIdMap, CelestialKindSaveData, SectorCelestialSaveData};
use crate::simulation::interaction_queue::InteractionQueue;
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{CelestialEntity, CelestialMass, SectorEntity};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Handle, Image, Name, Rot2};
use bevy::sprite::Sprite;
use common::components::celestials::{Celestial, GasGiant, Planet, Star};
use common::components::{ConstantOrbit, SectorWithCelestials, SelectableEntity};
use common::constants;
use common::types::polar_coordinates::PolarCoordinates;

fn get_sprite(kind: &CelestialKindSaveData, sprites: &SpriteHandles) -> Handle<Image> {
    match kind {
        CelestialKindSaveData::Star => sprites.star.clone(),
        CelestialKindSaveData::Terrestrial => sprites.planet.clone(),
        CelestialKindSaveData::GasGiant { .. } => sprites.planet.clone(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_celestial(
    commands: &mut Commands,
    sector_with_celestials: &mut SectorWithCelestials,
    celestial_id_map: &mut CelestialIdMap,
    sprites: &SpriteHandles,
    celestial_data: &SectorCelestialSaveData,
    sector_pos: Vec2,
    sector_entity: SectorEntity,
    orbit_mass: Option<CelestialMass>,
) {
    let simulation_transform =
        SimulationTransform::new(sector_pos + celestial_data.local_position, Rot2::IDENTITY);

    let entity = commands
        .spawn((
            Name::new(celestial_data.name.clone()),
            Sprite::from_image(get_sprite(&celestial_data.kind, sprites)),
            simulation_transform.as_bevy_transform(constants::z_layers::PLANET_AND_STARS),
            simulation_transform,
            SimulationScale::default(),
            Celestial {
                mass: celestial_data.mass,
                id: celestial_data.id,
            },
        ))
        .id();

    if let Some(orbit_mass) = orbit_mass {
        // Prevent the center object from receiving unnecessary orbit logic
        if celestial_data.local_position.length_squared() > 5.0 {
            let polar_coordinates =
                PolarCoordinates::from_cartesian(&celestial_data.local_position);
            commands
                .entity(entity)
                .insert(ConstantOrbit::new(polar_coordinates, &orbit_mass));
        }
    }

    match &celestial_data.kind {
        CelestialKindSaveData::Star => {
            commands
                .entity(entity)
                .insert((Star {}, SelectableEntity::Star));
            sector_with_celestials.add_star(commands, sector_entity, entity.into());
        }
        CelestialKindSaveData::Terrestrial => {
            commands
                .entity(entity)
                .insert((Planet {}, SelectableEntity::Celestial));
            sector_with_celestials.add_planet(commands, sector_entity, entity.into());
        }
        CelestialKindSaveData::GasGiant { resources } => {
            commands.entity(entity).insert((
                GasGiant {
                    resources: resources.clone(),
                },
                SelectableEntity::Celestial,
                InteractionQueue::new(constants::SIMULTANEOUS_PLANET_INTERACTIONS),
            ));
            sector_with_celestials.add_gas_giant(commands, sector_entity, entity.into());
        }
    };

    let celestial_entity = CelestialEntity::from(entity);
    celestial_id_map.insert(celestial_data.id, celestial_entity);
}
