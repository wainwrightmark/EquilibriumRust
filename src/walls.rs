use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::Point2;

pub const WINDOW_WIDTH: f32 = 360f32;
pub const WINDOW_HEIGHT: f32 = 640f32;
pub const WALL_WIDTH: f32 = 30f32;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls.system().after("main_setup").label("walls"));
    }
}

fn spawn_walls(mut commands: Commands, rapier_config: ResMut<RapierConfiguration>) {
    let scale = rapier_config.scale;

    //Spawn outer wall
    //Spawn top and bottom wall
    let shape_top_and_bottom_wall = shapes::Rectangle {
        extents: Vec2::new(WINDOW_WIDTH, WALL_WIDTH),
        origin: shapes::RectangleOrigin::Center,
    };

    //Spawn bottom wall
    let bottom_wall_pos: Point2<f32> = Point2::new(0.0, -WINDOW_HEIGHT / 2.0) / scale;
    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_top_and_bottom_wall,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TEAL),
                outline_mode: StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_top_and_bottom_wall.extents.x / scale / 2.0,
                shape_top_and_bottom_wall.extents.y / scale / 2.0,
            )
            .into(),
            position: bottom_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    //Spawn top wall
    let top_wall_pos: Point2<f32> = Point2::new(0.0, WINDOW_HEIGHT / 2.0)/ scale;
    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_top_and_bottom_wall,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TEAL),
                outline_mode: StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_top_and_bottom_wall.extents.x / scale / 2.0,
                shape_top_and_bottom_wall.extents.y / scale / 2.0,
            )
            .into(),
            position: top_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    //Spawn left and right wall
    let shape_left_and_right_wall = shapes::Rectangle {
        extents: Vec2::new(WALL_WIDTH, WINDOW_HEIGHT),
        origin: shapes::RectangleOrigin::Center,
    };

    //Spawn left wall
    let left_wall_pos: Point2<f32> = Point2::new(-WINDOW_WIDTH / 2.0, 0.0)/ scale;
    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_left_and_right_wall,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TEAL),
                outline_mode: StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_left_and_right_wall.extents.x / scale / 2.0,
                shape_left_and_right_wall.extents.y / scale / 2.0,
            )
            .into(),
            position: left_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    //Spawn right wall
    let right_wall_pos: Point2<f32> = Point2::new(WINDOW_WIDTH / 2.0, 0.0)/ scale;
    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape_left_and_right_wall,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TEAL),
                outline_mode: StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                shape_left_and_right_wall.extents.x / scale / 2.0,
                shape_left_and_right_wall.extents.y / scale / 2.0,
            )
            .into(),
            position: right_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
