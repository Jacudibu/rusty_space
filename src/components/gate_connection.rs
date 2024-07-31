use crate::components::Sector;
use crate::constants;
use crate::utils::GateEntity;
use bevy::math::Vec3;
use bevy::prelude::{Component, CubicBezier, CubicCurve, CubicGenerator, Vec2};

#[derive(Component)]
pub struct GateConnectionComponent {
    pub from: GateEntity,
    pub to: GateEntity,
    pub render_positions: Vec<Vec3>,
}

/// Marker Component to filter out connections in between gates which move
#[derive(Component)]
pub struct MovingGateConnection;

impl GateConnectionComponent {
    pub fn new(from: GateEntity, to: GateEntity, from_to_curve: &CubicCurve<Vec2>) -> Self {
        Self {
            from,
            to,
            render_positions: Self::calculate_render_positions(from_to_curve),
        }
    }

    pub fn calculate_render_positions(from_to_curve: &CubicCurve<Vec2>) -> Vec<Vec3> {
        from_to_curve
            .iter_positions(20)
            .map(|x| x.extend(constants::z_layers::GATE_CONNECTION))
            .collect()
    }

    pub fn calculate_curves_from_local_positions(
        from_sector: &Sector,
        from_pos: Vec2,
        to_sector: &Sector,
        to_pos: Vec2,
    ) -> (CubicCurve<Vec2>, CubicCurve<Vec2>) {
        let a = from_sector.world_pos + from_pos;
        let b = to_sector.world_pos + to_pos;

        Self::calculate_curves_from_global_positions(a, b)
    }

    pub fn calculate_curves_from_global_positions(
        a: Vec2,
        b: Vec2,
    ) -> (CubicCurve<Vec2>, CubicCurve<Vec2>) {
        let difference = a - b;
        let diff_rot = Vec2::new(-difference.y, difference.x) * 0.075;

        let a_curve = a - difference * 0.40 + diff_rot;
        let b_curve = b + difference * 0.40 - diff_rot;

        let ship_curve = Self::create_curve(a, a_curve, b_curve, b);
        let ship_curve_inverted = Self::create_curve(b, b_curve, a_curve, a);

        (ship_curve, ship_curve_inverted)
    }

    fn create_curve(a: Vec2, a_curve: Vec2, b_curve: Vec2, b: Vec2) -> CubicCurve<Vec2> {
        CubicBezier::new([[a, a_curve, b_curve, b]]).to_curve()
    }
}
