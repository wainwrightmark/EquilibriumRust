use bevy::ecs::event::Events;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use rand::seq::IteratorRandom;

use crate::*;
use crate::game_shape::GameShapeBody;

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_for_contacts)
            .add_system(check_for_win.after(check_for_contacts))
            .add_system(handle_new_game.after(check_for_win))
            .add_system_to_stage(CoreStage::PostUpdate, check_for_tower);
    }
}

const COUNTDOWN: f64 = 3.0;

pub fn handle_new_game(
    mut commands: Commands,
    mut new_game_events: EventReader<NewGameEvent>,
    draggables: Query<(Entity, With<Draggable>)>,
) {
    let mut first = true;
    for _ng in new_game_events.iter() {
        if !first {
            continue;
        }
        first = false;

        let mut shape_count = -1;
        for (e, _) in draggables.iter() {
            commands.entity(e).despawn();
            shape_count += 1;
        }
        shape_count += _ng.box_count_change; //create one more shape for new game

        let mut rng = rand::thread_rng();

        for _ in 0..=shape_count {
            //instead of random, give each shape its own place to spawn
            let shape = crate::game_shape::ALL_SHAPES.iter().choose(&mut rng).unwrap();

            let range_x = -100f32..100f32;
            let range_y = -100f32..100f32;

            let point = Vec2::new(rng.gen_range(range_x), rng.gen_range(range_y));

            let angle = rng.gen_range(0f32..std::f32::consts::TAU);

            create_shape(
                &mut commands,
                shape.clone(),
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
}

pub fn check_for_win(
    mut commands: Commands,
    mut win_timer: Query<(Entity, &WinTimer, &mut Transform)>,
    time: Res<Time>,
    mut new_game_events: EventWriter<NewGameEvent>,
) {
    if let Ok((timer_entity, timer, mut timer_transform)) = win_timer.get_single_mut() {
        let remaining = timer.win_time - time.elapsed_seconds_f64();

        if remaining <= 0f64 {
            //println!("Win - Despawn Win Timer {:?}", timer_entity);
            commands.entity(timer_entity).despawn();
            //println!("Win");
            new_game_events.send(NewGameEvent {
                box_count_change: 1,
            });
        } else {
            let new_scale = (remaining / COUNTDOWN) as f32;

            timer_transform.scale = Vec3::new(new_scale, new_scale, 1.0);
        }
    }
}

pub fn check_for_tower(
    mut commands: Commands,
    mut end_drag_events: EventReader<crate::DragEndedEvent>,
    win_timer: Query<&WinTimer>,
    time: Res<Time>,
    dragged: Query<With<Dragged>>,

    mut collision_events: ResMut<Events<CollisionEvent>>,
    rapier_context: Res<RapierContext>,
    walls: Query<(Entity, With<Wall>)>,
) {
    if !end_drag_events.iter().any(|_| true) {
        return;
    }

    //println!("Drag Ended");

    if !win_timer.is_empty() {
        //  println!("Win timer Exists");
        return; // no need to check, we're already winning
    }

    if !dragged.is_empty() {
        //println!("Something is dragged");
        return; //Something is being dragged so the player can't win yet
    }

    //Check for contacts
    for (wall, _) in walls.iter() {
        for contact in rapier_context.contacts_with(wall) {
            if contact.has_any_active_contacts() {
                return;
            }
        }
    }

    collision_events.clear();

    commands
        .spawn(WinTimer {
            win_time: time.elapsed_seconds_f64() + COUNTDOWN,
        })
        .insert(Transform {
            translation: Vec3::new(50.0, 200.0, 0.0),
            ..Default::default()
        })
        .insert(game_shape::circle::Circle{}.get_shape_bundle(
            100f32,
            ShapeAppearance {
                fill: Color::Hsla {
                    hue: (100f32),
                    saturation: (70f32),
                    lightness: (70f32),
                    alpha: (0.5),
                },
                stroke: Color::BLACK,
                line_width: 0f32,
            },
        ));

    //println!("Tower Built");
}

fn check_for_contacts(
    mut commands: Commands,
    win_timer: Query<(Entity, &WinTimer)>,
    mut collision_events: EventReader<CollisionEvent>,
    dragged: Query<With<Dragged>>,
) {
    if win_timer.is_empty() {
        return; // no need to check
    }

    let mut fail: Option<&str> = None;

    for _ie in collision_events.iter() {
        fail = Some("Intersection Found");
    }

    for _ in collision_events.iter() {
        fail = Some("Contact Found");
    }

    if fail.is_none() && !dragged.is_empty() {
        fail = Some("Something Dragged");
    }

    if let Some(_error_message) = fail {
        commands.entity(win_timer.single().0).despawn();
    }
}
