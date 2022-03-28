use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    shapes::{Circle, Polygon, Rectangle},
};
use bevy_rapier2d::prelude::*;
use itertools::*;
use nalgebra::Point2;
use rand::Rng;
use rand::prelude::ThreadRng;


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum GameShape {
    Circle,
    Cross,
    Triangle,
    Box,
}

impl GameShape {

    pub fn name(&self) -> String{
        match self{
            GameShape::Circle => String::from("Circle") ,
            GameShape::Cross => String::from("Cross"),
            GameShape::Triangle => String::from("Triangle"),
            GameShape::Box => String::from("Box"),
        }
    }

    pub fn default_fill_color(&self) -> Color{
        match self{
            GameShape::Circle => Color::hsla(0f32, 0.35, 0.45, 0.8),
            GameShape::Cross => Color::hsla(90f32, 0.35, 0.45, 0.8),
            GameShape::Triangle =>Color::hsla(180f32, 0.35, 0.45, 0.8),
            GameShape::Box => Color::hsla(270f32, 0.35, 0.45, 0.8),
        }
    }

    pub fn to_collider_shape(&self, shape_size: f32, physics_scale: f32) -> ColliderShape {
        match self {
            GameShape::Circle => {
                GameShape::circle_collider_shape(shape_size, physics_scale)
            }
            GameShape::Cross => {
                GameShape::cross_collider_shape(shape_size, physics_scale)
            }
            GameShape::Triangle => {
                GameShape::triangle_collider_shape(shape_size, physics_scale)
            }
            GameShape::Box => {
                GameShape::box_collider_shape(shape_size, physics_scale)
            }
        }
    }
    pub fn get_shapebundle(&self, shape_size: f32, appearance : ShapeAppearance) -> ShapeBundle {
        match self {
            GameShape::Circle => {
                GameShape::circle_shapebundle(shape_size, appearance)
            }
            GameShape::Cross => {
                GameShape::cross_shapebundle(shape_size, appearance)
            }
            GameShape::Triangle => {
                GameShape::triangle_shapebundle(shape_size, appearance)
            }
            GameShape::Box => {
                GameShape::box_shapebundle(shape_size, appearance)
            }
        }
    }

    fn cross_shapebundle(shape_size: f32, appearance : ShapeAppearance) -> ShapeBundle {
        {
            let u = shape_size / 3.0;
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

            GeometryBuilder::build_as(
                &geo,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(appearance.fill),
                    outline_mode: StrokeMode::new(appearance.stroke, appearance.line_width),
                },
                Transform::default(),
            )
        }
    }

    fn cross_collider_shape(shape_size: f32, physics_scale: f32) -> ColliderShape {
        ColliderShape::compound(vec![
            (
                Vec2::new(0f32, 0f32).into(),
                ColliderShape::cuboid(
                    shape_size / physics_scale / 6.0,
                    shape_size / physics_scale / 2.0,
                ),
            ),
            (
                Vec2::new(0f32, 0f32).into(),
                ColliderShape::cuboid(
                    shape_size / physics_scale / 2.0,
                    shape_size / physics_scale / 6.0,
                ),
            ),
        ])
    }

    fn box_geometry(shape_size: f32) -> Rectangle {
        shapes::Rectangle {
            extents: Vec2::new(shape_size, shape_size),
            origin: shapes::RectangleOrigin::Center,
        }
    }

    fn box_shapebundle(shape_size: f32, appearance : ShapeAppearance) -> ShapeBundle {
        GeometryBuilder::build_as(
            &GameShape::box_geometry(shape_size),
            DrawMode::Outlined {
                fill_mode: FillMode::color(appearance.fill),
                outline_mode: StrokeMode::new(appearance.stroke, appearance.line_width),
            },
            Transform::default(),
        )
    }

    fn box_collider_shape(shape_size: f32, physics_scale: f32) -> ColliderShape {
        let geo = GameShape::box_geometry(shape_size);
        ColliderShape::cuboid(
            geo.extents.x / physics_scale / 2.0,
            geo.extents.y / physics_scale / 2.0,
        )
    }

    fn circle_geometry(shape_size: f32) -> Circle {
        shapes::Circle {
            center: Vec2::new(0f32, 0f32),
            radius: shape_size / 2.0,
        }
    }
    fn circle_shapebundle(shape_size: f32, appearance : ShapeAppearance) -> ShapeBundle {
        GeometryBuilder::build_as(
            &GameShape::circle_geometry(shape_size),
            DrawMode::Outlined {
                fill_mode: FillMode::color(appearance.fill),
                outline_mode: StrokeMode::new(appearance.stroke, appearance.line_width),
            },
            Transform::default(),
        )
    }

    fn circle_collider_shape(shape_size: f32, physics_scale: f32) -> ColliderShape {
        let geo = GameShape::circle_geometry(shape_size);
        ColliderShape::ball(geo.radius / physics_scale)
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

    fn triangle_collider_shape(shape_size: f32, physics_scale: f32) -> ColliderShape {
        let geo = GameShape::triangle_geometry(shape_size);

        let r = ColliderShape::convex_hull(
            &geo.points
                .iter()
                .map(|v| Point2::new(v.x / physics_scale, v.y / physics_scale))
                .collect_vec(),
        );
        return r.unwrap();
    }

    fn triangle_shapebundle(shape_size: f32, appearance : ShapeAppearance) -> ShapeBundle {
        GeometryBuilder::build_as(
            &GameShape::triangle_geometry(shape_size),
            DrawMode::Outlined {
                fill_mode: FillMode::color(appearance.fill),
                outline_mode: StrokeMode::new(appearance.stroke, appearance.line_width),
            },
            Transform::default(),
        )
    }
}


pub struct ShapeAppearance{
    pub fill: Color,
    pub stroke: Color,
    pub line_width: f32
}

impl Default for ShapeAppearance{
    fn default() -> Self {
        Self { fill: Color::WHITE, stroke: Color::BLACK, line_width: 2.0 }
    }
}


pub fn game_shapes() -> Vec<GameShape>
{
    vec![
        GameShape::Circle ,
        GameShape::Triangle,
        GameShape::Box ,
        GameShape::Cross ,
        
        ]
} 

pub fn get_random_shape(rng: &mut ThreadRng) -> GameShape{
    let shapes = game_shapes();
    let length = shapes.len();
    let index = rng.gen_range(0..length);
    let r = shapes[index];
    return r;
}
