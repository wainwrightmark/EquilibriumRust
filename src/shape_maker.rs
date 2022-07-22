use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::*;

use rand::Rng;

pub const SHAPE_SIZE: f32 = 60f32;

pub fn create_game(mut commands: Commands) {
    create_boxes(&mut commands);

    create_foundations(&mut commands, &GameShape::Box);
}

pub fn create_foundations(commands: &mut Commands, shape: &GameShape) {
    let x = 0f32;
    let y = SHAPE_SIZE - (crate::WINDOW_HEIGHT / 2.0);

    create_shape(
        commands,
        shape,
        SHAPE_SIZE,
        Vec2::new(x, y),
        0f32,
        false,
        ShapeAppearance {
            fill: (Color::GRAY),
            ..Default::default()
        },
    );
}

pub fn create_boxes(commands: &mut Commands) {
    let mut rng = rand::thread_rng();

    for shape in crate::game_shape::game_shapes() {
        let rangex = -100f32..100f32;
        let rangey = -100f32..100f32;

        let point = Vec2::new(rng.gen_range(rangex), rng.gen_range(rangey));

        let angle = rng.gen_range(0f32..std::f32::consts::TAU);

        create_shape(
            commands,
            &shape,
            SHAPE_SIZE,
            point,
            angle,
            true,
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
    dynamic: bool,
    appearance: ShapeAppearance,
) {
    let collider_shape = shape.to_collider_shape(shape_size);
    let transform: Transform = Transform {
        translation: position.extend(0.0),
        rotation: Quat::from_rotation_x(angle),
        scale: Vec3::ONE,
    };

    let rbb: RigidBody = if dynamic {
        RigidBody::Dynamic
    } else {
        RigidBody::Fixed
    };

    let mut entity_builder = commands.spawn();
    let name = shape.name();

    entity_builder
        .insert_bundle(shape.get_shapebundle(shape_size, appearance))
        .insert(rbb)
        .insert(collider_shape)
        .insert(transform)
        .insert(Name::new(name));

    if dynamic {
        entity_builder.insert(crate::Draggable {});
    } else {
        entity_builder.insert(Foundation {});
    }

    //println!("Spawn {:?} {:?}", entity_builder.id(), shape);
}
