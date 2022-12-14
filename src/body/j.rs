use bevy::prelude::{Vec2, Transform};
use bevy_prototype_lyon::{shapes::Polygon, prelude::GeometryBuilder};
use bevy_rapier2d::prelude::Collider;
use itertools::Itertools;
use crate::shape_appearance::*;

use super::Body;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct J{   
}

impl Body for J{
    fn name(&self) -> &'static str {
        "J Shape"
    }
    fn to_collider_shape(self, shape_size: f32) -> bevy_rapier2d::prelude::Collider {
        let u = shape_size * 0.25;

        Collider::compound(vec![
            (Vec2::new(0.0 , u * -1.), 0.0, Collider::cuboid(u * 2.0, u)),
            (Vec2::new(u, u ), 0.0, Collider::cuboid(u, u * 3.0)),
        ])
    }

    fn get_shape_bundle(self, shape_size: f32, appearance: ShapeAppearance) -> bevy_prototype_lyon::entity::ShapeBundle {
        let u = shape_size * 0.5;
        let offset = Vec2::new(u *-1., u * -1.);
            let geo = Polygon {
                points: [
                    Vec2::new(0.0, 0.0),
                    Vec2::new(2.0 * u, 0.0),
                    Vec2::new(2.0 * u, 3.0 * u),
                    Vec2::new(1.0 * u, 3.0 * u),
                    Vec2::new(1.0 * u, 1.0 * u),
                    Vec2::new(0.0 * u, 1.0 * u),
                ]
                .into_iter()
                .map(|p| p + offset)
                .collect_vec(),
                closed: true,
            };

            GeometryBuilder::build_as(&geo, appearance.into(), Transform::default())
    }
}