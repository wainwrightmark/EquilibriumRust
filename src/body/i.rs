use bevy::prelude::{Vec2, Transform};
use bevy_prototype_lyon::{shapes::{ Rectangle, self}, prelude::GeometryBuilder};
use bevy_rapier2d::prelude::Collider;

use super::Body;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct I{   
}

fn box_geometry(shape_size: f32) -> Rectangle {
    shapes::Rectangle {
        extents: Vec2::new(shape_size * 2.0, shape_size * 0.5),
        origin: shapes::RectangleOrigin::Center,
    }
}

impl Body for I{
    fn name(&self) -> &'static str {
        "I Shape"
    }

    fn to_collider_shape(self, shape_size: f32) -> bevy_rapier2d::prelude::Collider {
        let geo = box_geometry(shape_size);
        Collider::cuboid(geo.extents.x / 2.0, geo.extents.y / 2.0)
    }

    fn get_shape_bundle(self, shape_size: f32, appearance: crate::shape_appearance::ShapeAppearance) -> bevy_prototype_lyon::entity::ShapeBundle {
        GeometryBuilder::build_as(
            &box_geometry(shape_size),
            appearance.into(),
            Transform::default(),
        )
    }
}