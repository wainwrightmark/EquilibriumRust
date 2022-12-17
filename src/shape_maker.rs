use bevy::prelude::*;
use bevy_prototype_lyon::prelude::FillMode;
use bevy_rapier2d::prelude::*;
use itertools::Itertools;

use crate::*;

use rand::{seq::SliceRandom, Rng};

pub const SHAPE_SIZE: f32 = 50f32;
pub const INITIAL_SHAPES: usize = 2;
pub const MAX_SHAPES: usize = 36;

pub fn create_game(mut commands: Commands) {
    create_n_boxes(&mut commands, INITIAL_SHAPES);
}

pub fn create_n_boxes(commands: &mut Commands, n: usize) {
    let mut rng = rand::thread_rng();

    const COLS: usize = 6;
    let mut positions = (0..MAX_SHAPES).collect_vec();
    positions.shuffle(&mut rng);

    for i in 0..n {
        let shape = crate::game_shape::ALL_SHAPES.choose(&mut rng).unwrap();
        let i = positions[i];
        let left = SHAPE_SIZE * (COLS as f32) / 2.;

        let x = ((i % COLS) as f32) * SHAPE_SIZE - left;
        let y = (((i / COLS) as f32) * SHAPE_SIZE) - left;
        //let range_x = -100f32..100f32;
        //let range_y = -100f32..100f32;

        let point = Vec2::new(x, y);
        let angle = rng.gen_range(0f32..std::f32::consts::TAU);

        create_shape(
            commands,
            shape.clone(),
            SHAPE_SIZE,
            point,
            angle,
            DrawMode::Outlined {
                fill_mode: FillMode::color(shape.default_fill_color()),
                outline_mode: StrokeMode {
                    options: StrokeOptions::DEFAULT,
                    color: Color::GRAY,
                },
            }, // ShapeAppearance {
               //     fill: (shape.default_fill_color()),
               //     ..Default::default()
               // },
        );
    }
}

pub fn create_shape(
    commands: &mut Commands,
    game_shape: game_shape::GameShape,
    shape_size: f32,
    position: Vec2,
    angle: f32,
    draw_mode: DrawMode,
) {
    let collider_shape = game_shape.body.to_collider_shape(shape_size);
    let transform: Transform = Transform {
        translation: position.extend(0.0),
        rotation: Quat::from_rotation_x(angle),
        scale: Vec3::ONE,
    };

    let rbb = RigidBody::Dynamic;

    commands
        .spawn(game_shape.body.get_shape_bundle(shape_size, draw_mode))
        .insert(rbb)
        .insert(collider_shape)
        .insert(transform)
        .insert(crate::Draggable {});
}
