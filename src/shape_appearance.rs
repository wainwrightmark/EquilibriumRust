use bevy::prelude::*;
use bevy_prototype_lyon::prelude::FillMode;
use bevy_prototype_lyon::prelude::*;




#[derive(Debug, Copy, Clone)]
pub struct ShapeAppearance {
    pub fill: Color,
    pub stroke: Color,
    pub line_width: f32,
}

impl From<ShapeAppearance> for DrawMode {
    fn from(val: ShapeAppearance) -> Self {
        DrawMode::Fill(FillMode::color(val.fill))
    }
}

impl Default for ShapeAppearance {
    fn default() -> Self {
        Self {
            fill: Color::WHITE,
            stroke: Color::BLACK,
            line_width: 2.0,
        }
    }
}