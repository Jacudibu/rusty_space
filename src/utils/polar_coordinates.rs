use bevy::prelude::Vec2;

/// Represents a position in PolarCoordinates.
pub struct PolarCoordinates {
    /// The radial coordinate r, indicating our distance from the pole.
    pub radial_distance: f32,
    /// The polar angle. 0 when pointing to the right, increasing counterclockwise up to 360 after going full circle.
    pub angle: f32,
}

impl PolarCoordinates {
    /// Converts a regular position represented as a [Vec2] into [PolarCoordinates].
    pub fn from_cartesian(pos: &Vec2) -> Self {
        let mut angle_in_radians = pos.y.atan2(pos.x);

        // Ensure we are in [0,2pi[ rather than [-pi,pi[
        if angle_in_radians < 0.0 {
            angle_in_radians += std::f32::consts::TAU;
        }

        Self {
            radial_distance: (pos.x * pos.x + pos.y * pos.y).sqrt(),
            angle: angle_in_radians.to_degrees(),
        }
    }

    /// Converts self into a [Vec2]
    pub fn to_cartesian(&self) -> Vec2 {
        Vec2 {
            x: self.radial_distance * self.angle.to_radians().cos(),
            y: self.radial_distance * self.angle.to_radians().sin(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    // TODO: Extract into some type of test crate or use approx or something
    macro_rules! assert_floating_eq {
        ($x:expr, $y:expr, $d:expr) => {
            if $x - $y > $d || $y - $x > $d {
                panic!(
                    r#"assertion failed: `(left == right (with max difference < {:?})`
   left: `{:?}`,
  right: `{:?}`"#,
                    $d, $x, $y
                );
            }
        };
    }

    #[rstest]
    #[case(11.0, 55.0)]
    #[case(-22.0, 66.0)]
    #[case(33.0, -77.0)]
    #[case(-44.0, -88.0)]
    fn round_trip(#[case] x: f32, #[case] y: f32) {
        let vec = Vec2::new(x, y);
        let polar = PolarCoordinates::from_cartesian(&vec);
        let back = polar.to_cartesian();
        assert_floating_eq!(vec.x, back.x, 0.0001);
        assert_floating_eq!(vec.y, back.y, 0.0001);
    }
}
