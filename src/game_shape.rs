use bevy::prelude::*;
use bevy_prototype_lyon::prelude::FillMode;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    shapes::{Circle, Polygon, Rectangle},
};
use bevy_rapier2d::prelude::*;
use itertools::*;
use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum GameShape {
    Circle,
    Cross,
    Triangle,
    Box,
    Ell,
}

impl GameShape {
    pub fn name(self) -> &'static str {
        match self {
            GameShape::Circle => "Circle",
            GameShape::Cross => "Cross",
            GameShape::Triangle => "Triangle",
            GameShape::Box => "Box",
            GameShape::Ell => "Ell",
        }
    }

    pub fn default_fill_color(self) -> Color {
        match self {
            GameShape::Circle => Color::hsla(0f32, 0.35, 0.45, 0.8),
            GameShape::Cross => Color::hsla(60f32, 0.35, 0.45, 0.8),
            GameShape::Triangle => Color::hsla(120f32, 0.35, 0.45, 0.8),
            GameShape::Box => Color::hsla(180f32, 0.35, 0.45, 0.8),
            GameShape::Ell => Color::hsla(240f32, 0.35, 0.45, 0.8),
        }
    }

    pub fn to_collider_shape(self, shape_size: f32) -> Collider {
        match self {
            GameShape::Circle => GameShape::circle_collider_shape(shape_size),
            GameShape::Cross => GameShape::cross_collider_shape(shape_size),
            GameShape::Triangle => GameShape::triangle_collider_shape(shape_size),
            GameShape::Box => GameShape::box_collider_shape(shape_size),
            GameShape::Ell => GameShape::ell_collider_shape(shape_size),
        }
    }
    pub fn get_shape_bundle(self, shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle {
        match self {
            GameShape::Circle => GameShape::circle_shapebundle(shape_size, appearance),
            GameShape::Cross => GameShape::cross_shapebundle(shape_size, appearance),
            GameShape::Triangle => GameShape::triangle_shapebundle(shape_size, appearance),
            GameShape::Box => GameShape::box_shapebundle(shape_size, appearance),
            GameShape::Ell => GameShape::ell_shapebundle(shape_size, appearance),
        }
    }

    fn cross_shapebundle(shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle {
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

            GeometryBuilder::build_as(&geo, appearance.into(), Transform::default())
        }
    }

    fn cross_collider_shape(shape_size: f32) -> Collider {
        Collider::compound(vec![
            (
                Vec2::ZERO,
                0.0,
                Collider::cuboid(shape_size / 6.0, shape_size / 2.0),
            ),
            (
                Vec2::ZERO,
                0.0,
                Collider::cuboid(shape_size / 2.0, shape_size / 6.0),
            ),
        ])
    }

    fn ell_shapebundle(shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle {
        {
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

    fn ell_collider_shape(shape_size: f32) -> Collider {
        let u = shape_size / 6.0;

        Collider::compound(vec![
            (Vec2::new(u * 2.0, u), 0.0, Collider::cuboid(u * 2.0, u)),
            (Vec2::new(u, u * 3.0), 0.0, Collider::cuboid(u, u * 3.0)),
        ])
    }

    fn box_geometry(shape_size: f32) -> Rectangle {
        shapes::Rectangle {
            extents: Vec2::new(shape_size / 2.0, shape_size / 2.0),
            origin: shapes::RectangleOrigin::Center,
        }
    }

    fn box_shapebundle(shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle {
        GeometryBuilder::build_as(
            &GameShape::box_geometry(shape_size),
            appearance.into(),
            Transform::default(),
        )
    }

    fn box_collider_shape(shape_size: f32) -> Collider {
        let geo = GameShape::box_geometry(shape_size);
        Collider::cuboid(geo.extents.x / 2.0, geo.extents.y / 2.0)
    }

    fn circle_geometry(shape_size: f32) -> Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: shape_size / 2.0,
        }
    }
    fn circle_shapebundle(shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle {
        GeometryBuilder::build_as(
            &GameShape::circle_geometry(shape_size),
            appearance.into(),
            Transform::default(),
        )
    }

    fn circle_collider_shape(shape_size: f32) -> Collider {
        let geo = GameShape::circle_geometry(shape_size);
        Collider::ball(geo.radius)
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

    fn triangle_collider_shape(shape_size: f32) -> Collider {
        let geo = GameShape::triangle_geometry(shape_size);

        let r =
            Collider::convex_hull(&geo.points.iter().map(|v| Vect::new(v.x, v.y)).collect_vec());
        r.unwrap()
    }

    fn triangle_shapebundle(shape_size: f32, appearance: ShapeAppearance) -> ShapeBundle {
        GeometryBuilder::build_as(
            &GameShape::triangle_geometry(shape_size),
            appearance.into(),
            Transform::default(),
        )
    }
}

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

pub const ALLGAMESHAPES: [GameShape; 5] = [
    GameShape::Circle,
    GameShape::Triangle,
    GameShape::Box,
    GameShape::Cross,
    GameShape::Ell,
];

pub fn get_random_shape(rng: &mut ThreadRng) -> GameShape {
    let index = rng.gen_range(0..ALLGAMESHAPES.len());

    ALLGAMESHAPES[index]
}
