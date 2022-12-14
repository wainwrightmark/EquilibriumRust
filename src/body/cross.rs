use bevy::prelude::{Transform, Vec2};
use bevy_prototype_lyon::{prelude::GeometryBuilder, shapes::Polygon};
use bevy_rapier2d::prelude::Collider;
use itertools::Itertools;

use super::Body;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Cross {}

impl Body for Cross {
    fn name(&self) -> &'static str {
        "Cross"
    }

    fn to_collider_shape(self, shape_size: f32) -> bevy_rapier2d::prelude::Collider {
        let u = shape_size * f32::sqrt(0.2) / 2.0;
        Collider::compound(vec![
            (Vec2::ZERO, 0.0, Collider::cuboid(u, 3. * u)),
            (Vec2::ZERO, 0.0, Collider::cuboid(3. * u, u)),
        ])
    }

    fn get_shape_bundle(
        self,
        shape_size: f32,
        appearance: crate::shape_appearance::ShapeAppearance,
    ) -> bevy_prototype_lyon::entity::ShapeBundle {
        let u = shape_size * f32::sqrt(0.2);
        let offset = Vec2::new(1.5 * u, 1.5 * u);
        let geo = Polygon {
            points: [
                Vec2::new(u, 0.0),
                Vec2::new(2.0 * u, 0.0),
                Vec2::new(2.0 * u, u),
                Vec2::new(3.0 * u, u),
                Vec2::new(3.0 * u, 2.0 * u),
                Vec2::new(2.0 * u, 2.0 * u),
                Vec2::new(2.0 * u, 3.0 * u),
                Vec2::new(1.0 * u, 3.0 * u),
                Vec2::new(1.0 * u, 2.0 * u),
                Vec2::new(0.0, 2.0 * u),
                Vec2::new(0.0, u),
                Vec2::new(u, u),
            ]
            .iter()
            .map(|p| *p - offset)
            .collect_vec(),
            closed: true,
        };

        GeometryBuilder::build_as(&geo, appearance.into(), Transform::default())
    }
}
