use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::*;

use rand::Rng;

pub const SHAPE_SIZE: f32 = 60f32;

pub fn create_game(mut commands: Commands) {
    create_boxes(&mut commands);
}

pub fn create_boxes(commands: &mut Commands) {
    let mut rng = rand::thread_rng();

    for shape in crate::game_shape::ALLGAMESHAPES {
        let range_x = -100f32..100f32;
        let range_y = -100f32..100f32;

        let point = Vec2::new(rng.gen_range(range_x), rng.gen_range(range_y));

        let angle = rng.gen_range(0f32..std::f32::consts::TAU);

        create_shape(
            commands,
            &shape,
            SHAPE_SIZE,
            point,
            angle,
            ShapeAppearance {
                fill: (shape.default_fill_color()),
                ..Default::default()
            },
        );
    }
}

pub fn create_shape(
    commands: &mut Commands,
    shape: &GameShape,
    shape_size: f32,
    position: Vec2,
    angle: f32,
    appearance: ShapeAppearance,
) {
    let collider_shape = shape.to_collider_shape(shape_size);
    let transform: Transform = Transform {
        translation: position.extend(0.0),
        rotation: Quat::from_rotation_x(angle),
        scale: Vec3::ONE,
    };

    let rbb = RigidBody::Dynamic;

    let name = shape.name().to_string();

    commands
        .spawn(shape.get_shape_bundle(shape_size, appearance))
        .insert(rbb)
        .insert(collider_shape)
        .insert(transform)
        .insert(Name::new(name))
        .insert(crate::Draggable {
            game_shape: shape.clone(),
        });
}
