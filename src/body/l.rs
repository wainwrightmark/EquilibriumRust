use bevy::prelude::{Vec2, Transform};
use bevy_prototype_lyon::{shapes::Polygon, prelude::GeometryBuilder};
use bevy_rapier2d::prelude::Collider;
use itertools::Itertools;

use super::Body;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct L{   
}

impl Body for L{
    fn name(&self) -> &'static str {
        "L Shape"
    }

    fn hue(&self) -> f32 {
        240f32
    }

    fn to_collider_shape(self, shape_size: f32) -> bevy_rapier2d::prelude::Collider {
        let u = shape_size / 6.0;

        Collider::compound(vec![
            (Vec2::new(u * 2.0, u), 0.0, Collider::cuboid(u * 2.0, u)),
            (Vec2::new(u, u * 3.0), 0.0, Collider::cuboid(u, u * 3.0)),
        ])
    }

    fn get_shape_bundle(self, shape_size: f32, appearance: crate::game_shape::ShapeAppearance) -> bevy_prototype_lyon::entity::ShapeBundle {
        let u = shape_size / 3.0;
            //let offset = Vec2::new(1.5 * u, 1.5 * u);
            let geo = Polygon {
                points: [
                    Vec2::new(0.0, 0.0),
                    Vec2::new(2.0 * u, 0.0),
                    Vec2::new(2.0 * u, 1.0 * u),
                    Vec2::new(1.0 * u, 1.0 * u),
                    Vec2::new(1.0 * u, 3.0 * u),
                    Vec2::new(0.0 * u, 3.0 * u),
                ]
                .into_iter()
                //.map(|p| *p - offset)
                .collect_vec(),
                closed: true,
            };

            GeometryBuilder::build_as(&geo, appearance.into(), Transform::default())
    }
}