use std::fmt::Debug;

use bevy::prelude::Color;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_rapier2d::prelude::Collider;
use strum::{Display, EnumCount, EnumIter};

use crate::shape_appearance::ShapeAppearance;

pub mod circle;
pub mod cross;
pub mod i;
pub mod j;
pub mod l;
pub mod square;
pub mod triangle;

pub use circle::*;
pub use cross::*;
pub use i::*;
pub use j::*;
pub use l::*;
pub use square::*;
pub use triangle::*;

const SATURATION: f32 = 0.35;
const LIGHTNESS: f32 = 0.45;
const ALPHA: f32 = 0.8;

#[enum_delegate::register]
pub trait Body {
    fn name(&self) -> &'static str;

    fn to_collider_shape(self, shape_size: f32) -> Collider;
    fn get_shape_bundle(self, shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle;
}

#[enum_delegate::implement(Body)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Display, EnumIter, EnumCount)]
pub enum GameShape {
    Circle(Circle),
    Cross(Cross),
    I(I),
    L(L),
    J(J),
    Square(Square),
    Triangle(RightIsoscelesTriangle),
}

impl GameShape {
    pub fn default_fill_color(&self) -> Color {
        let hue = self.index() * 360 / GameShape::COUNT;
        Color::hsla(hue as f32, SATURATION, LIGHTNESS, ALPHA)
    }

    fn index(&self) -> usize {
        match self {
            GameShape::Circle(_) => 0,
            GameShape::Cross(_) => 1,
            GameShape::L(_) => 2,
            GameShape::I(_) => 3,
            GameShape::J(_) => 4,
            GameShape::Square(_) => 5,
            GameShape::Triangle(_) => 6,
        }
    }
}
