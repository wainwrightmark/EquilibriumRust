use std::fmt::Debug;

use bevy::prelude::Color;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_rapier2d::prelude::Collider;
use strum::{EnumIter, Display, EnumCount};

use crate::shape_appearance::ShapeAppearance;

pub mod circle;
pub mod cross;
pub mod l;
pub mod square;
pub mod triangle;

pub use circle::*;
pub use cross::*;
pub use l::*;
pub use square::*;
pub use triangle::*;

const SATURATION: f32 = 0.35;
const LIGHTNESS: f32 = 0.45;
const ALPHA: f32 = 0.8;

#[enum_delegate::register]
pub trait Body  {
    fn name(&self) -> &'static str;
    fn default_fill_color(&self) -> Color {
        Color::hsla(self.hue(), SATURATION, LIGHTNESS, ALPHA)
    }

    fn hue(&self) -> f32;
    fn to_collider_shape(self, shape_size: f32) -> Collider;
    fn get_shape_bundle(self, shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle;
}


#[enum_delegate::implement(Body)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Display, EnumIter, EnumCount)]
pub enum GameShape {
    Circle(Circle),
    Cross(Cross),
    L(L),
    Square(Square),
    Triangle(Triangle),
}
