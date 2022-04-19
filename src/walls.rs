use crate::*;
use nalgebra::Point2;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls.system().after("main_setup").label("walls"));
    }
}

fn spawn_walls(mut commands: Commands, rapier_config: ResMut<RapierConfiguration>) {
    let scale = rapier_config.scale;

    let color = Color::GRAY;
    const OFFSET: f32 = crate::WALL_WIDTH / 2.0;
    const EXTRA_WIDTH: f32 = crate::WALL_WIDTH * 2.0;

    let bottom_wall_pos: Point2<f32> =
        Point2::new(0.0, -crate::WINDOW_HEIGHT / 2.0 - OFFSET) / scale;
    let top_wall_pos: Point2<f32> = Point2::new(0.0, crate::WINDOW_HEIGHT / 2.0 + OFFSET) / scale;
    let left_wall_pos: Point2<f32> = Point2::new(-crate::WINDOW_WIDTH / 2.0 - OFFSET, 0.0) / scale;
    let right_wall_pos: Point2<f32> = Point2::new(crate::WINDOW_WIDTH / 2.0 + OFFSET, 0.0) / scale;

    spawn_wall(
        &mut commands,
        scale,
        bottom_wall_pos,
        crate::WINDOW_WIDTH + EXTRA_WIDTH,
        crate::WALL_WIDTH,
        color,
        "Bottom-Wall".to_string(),
    );
    spawn_wall(
        &mut commands,
        scale,
        top_wall_pos,
        crate::WINDOW_WIDTH + EXTRA_WIDTH,
        crate::WALL_WIDTH,
        color,
        "Top-Wall".to_string(),
    );

    spawn_wall(
        &mut commands,
        scale,
        left_wall_pos,
        crate::WALL_WIDTH,
        crate::WINDOW_HEIGHT,
        color,
        "Left-Wall".to_string(),
    );
    spawn_wall(
        &mut commands,
        scale,
        right_wall_pos,
        crate::WALL_WIDTH,
        crate::WINDOW_HEIGHT,
        color,
        "Right-Wall".to_string(),
    );
}

fn spawn_wall(
    commands: &mut Commands,
    physics_scale: f32,
    point: Point2<f32>,
    width: f32,
    height: f32,
    color: Color,
    name: String,
) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(width, height),
        origin: shapes::RectangleOrigin::Center,
    };
    let collider_shape1 = ColliderShape::cuboid(
        shape.extents.x / physics_scale / 2.0,
        shape.extents.y / physics_scale / 2.0,
    );
    let collider_shape2 = ColliderShape::cuboid(
        shape.extents.x / physics_scale / 2.0,
        shape.extents.y / physics_scale / 2.0,
    );

    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(color),
                outline_mode: StrokeMode::color(color),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: point.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: collider_shape1.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Name::new(name.to_string()))
        .insert(Wall {})
        .with_children(|f| {
            f.spawn_bundle(ColliderBundle {
                shape: collider_shape2.into(),
                flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
                collider_type: ColliderType::Sensor.into(),

                ..Default::default()
            })
            .insert(Name::new(name));
        });
}
