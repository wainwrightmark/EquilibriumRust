use bevy::prelude::{Vec2, Transform};
use bevy_prototype_lyon::{shapes::{ self}, prelude::GeometryBuilder};
use bevy_rapier2d::prelude::Collider;

use super::Body;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Circle{   
}

fn circle_geometry(shape_size: f32) -> bevy_prototype_lyon::shapes::Circle {
    shapes::Circle {
        center: Vec2::ZERO,
        radius: shape_size / 2.0,
    }
}

impl Body for Circle{
    fn name(&self) -> &'static str {
        "Circle"
    }

    fn hue(&self) -> f32 {
        0f32
    }

    fn to_collider_shape(self, shape_size: f32) -> bevy_rapier2d::prelude::Collider {
        let geo = circle_geometry(shape_size);
        Collider::ball(geo.radius)
    }

    fn get_shape_bundle(self, shape_size: f32, appearance: crate::shape_appearance::ShapeAppearance) -> bevy_prototype_lyon::entity::ShapeBundle {
        GeometryBuilder::build_as(
            &circle_geometry(shape_size),
            appearance.into(),
            Transform::default(),
        )
    }
}