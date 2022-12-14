use bevy::prelude::{Vec2, Transform};
use bevy_prototype_lyon::{shapes::{Polygon, self}, prelude::GeometryBuilder};
use bevy_rapier2d::prelude::{Collider, Vect};
use itertools::Itertools;

use super::Body;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Triangle{   
}

fn triangle_geometry(shape_size: f32) -> Polygon {
    let p = shape_size / 3.0;
    shapes::Polygon {
        closed: true,
        points: vec![
            Vec2::new(-p, -p),
            Vec2::new(-p, 2.0 * p),
            Vec2::new(2.0 * p, -p),
        ],
    }
}

impl Body for Triangle{
    fn name(&self) -> &'static str {
        "Triangle"
    }

    fn hue(&self) -> f32 {
        120f32
    }

    fn to_collider_shape(self, shape_size: f32) -> bevy_rapier2d::prelude::Collider {
        let geo = triangle_geometry(shape_size);

        let r =
            Collider::convex_hull(&geo.points.iter().map(|v| Vect::new(v.x, v.y)).collect_vec());
        r.unwrap()
    }

    fn get_shape_bundle(self, shape_size: f32, appearance: crate::shape_appearance::ShapeAppearance) -> bevy_prototype_lyon::entity::ShapeBundle {
        GeometryBuilder::build_as(
            &triangle_geometry(shape_size),
            appearance.into(),
            Transform::default(),
        )
    }
}