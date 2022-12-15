use std::{fmt::Debug, sync::Arc};

use bevy::{prelude::Color, render::once_cell::sync::Lazy};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_rapier2d::prelude::Collider;

use crate::shape_appearance::ShapeAppearance;

pub mod circle;

pub mod polygon;
pub mod triangle;

pub use circle::*;

pub use polygon::*;
pub use triangle::*;

const SATURATION: f32 = 0.35;
const LIGHTNESS: f32 = 0.45;
const ALPHA: f32 = 0.8;

pub trait GameShapeBody: Send + Sync {
    fn to_collider_shape(&self, shape_size: f32) -> Collider;
    fn get_shape_bundle(&self, shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle;
}

#[derive(Clone)]
pub struct GameShape {
    pub name: &'static str,
    pub body: Arc<dyn GameShapeBody>,
    pub index: usize,
}

impl GameShape {
    pub fn default_fill_color(&self) -> Color {
        let hue = self.index * 360 / ALL_SHAPES.len();
        Color::hsla(hue as f32, SATURATION, LIGHTNESS, ALPHA)
    }
}

impl Debug for GameShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Display for GameShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub const SHAPE_COUNT: usize = 7;
pub static ALL_SHAPES: Lazy<[GameShape; SHAPE_COUNT]> = Lazy::new(|| {
    let mut index = 0;
    let arr1: [(&'static str, Arc<dyn GameShapeBody>); SHAPE_COUNT] = [
        ("Circle", Arc::new(Circle {})),
        ("Square", Arc::new(SQUARE)),
        ("Cross", Arc::new(CROSS)),
        ("Triangle", Arc::new(TRIANGLE)),
        ("J", Arc::new(J_POLYOMINO)),
        ("L", Arc::new(L_POLYOMINO)),
        ("I", Arc::new(I_POLYOMINO)),
    ];

    arr1.map(|(name, body)| {
        let r = GameShape { name, body, index };
        index += 1;
        r
    })
});

const SQUARE: PolygonBody<1, 4> = PolygonBody(&[(0, 0), (1, 0), (1, 1), (0, 1)]);
const CROSS: PolygonBody<5, 12> = PolygonBody(&[
    (1, 0),
    (2, 0),
    (2, 1),
    (3, 1),
    (3, 2),
    (2, 2),
    (2, 3),
    (1, 3),
    (1, 2),
    (0, 2),
    (0, 1),
    (1, 1),
]);

const TRIANGLE: PolygonBody<4, 3> = PolygonBody(&[(-1, -1), (-1, 2), (2, -1)]);

const J_POLYOMINO: PolygonBody<4, 6> =
    PolygonBody(&[(0, 0), (2, 0), (2, 3), (1, 3), (1, 1), (0, 1)]);
const L_POLYOMINO: PolygonBody<4, 6> =
    PolygonBody(&[(0, 0), (2, 0), (2, 1), (1, 1), (1, 3), (0, 3)]);
const I_POLYOMINO: PolygonBody<5, 4> = PolygonBody(&[(0, 0), (1, 0), (1, 5), (0, 5)]);
