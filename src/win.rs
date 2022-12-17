use std::borrow::BorrowMut;

use bevy::ecs::event::Events;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game_shape::GameShapeBody;
use crate::*;

#[derive(Component)]
pub struct WinTimer {
    pub win_time: f64,
}

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_for_contacts)
            .add_system(check_for_win.after(check_for_contacts))
            .add_system(handle_change_level.after(check_for_win))
            .add_system_to_stage(CoreStage::PostUpdate, check_for_tower);
    }
}

const COUNTDOWN: f64 = 1.0;
const TIME_SCALE: f32 = 3.;

fn scale_time(mut rapier_config: ResMut<RapierConfiguration>, scale: f32) {
    match rapier_config.timestep_mode.borrow_mut() {
        TimestepMode::Fixed { dt, substeps: _ } => {
            *dt = scale / 60.;
        }
        TimestepMode::Variable {
            max_dt,
            time_scale,
            substeps: _,
        } => {
            *time_scale = scale;
            *max_dt = scale / 60.;
        }
        TimestepMode::Interpolated {
            dt,
            time_scale,
            substeps: _,
        } => {
            *time_scale = scale;
            *dt = scale / 60.;
        }
    }
}

pub fn check_for_win(
    mut commands: Commands,
    mut win_timer: Query<(Entity, &WinTimer, &mut Transform)>,
    time: Res<Time>,
    mut new_game_events: EventWriter<ChangeLevelEvent>,
    rapier_config: ResMut<RapierConfiguration>,
) {
    if let Ok((timer_entity, timer, mut timer_transform)) = win_timer.get_single_mut() {
        let remaining = timer.win_time - time.elapsed_seconds_f64();

        if remaining <= 0f64 {
            scale_time(rapier_config, 1.);

            commands.entity(timer_entity).despawn();

            new_game_events.send(ChangeLevelEvent::Next);
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
    mut rapier_config: ResMut<RapierConfiguration>,
    walls: Query<(Entity, With<Wall>)>,
) {
    if !end_drag_events.iter().any(|_| true) {
        return;
    }
    if !win_timer.is_empty() {
        return; // no need to check, we're already winning
    }

    if !dragged.is_empty() {
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

    scale_time(rapier_config, TIME_SCALE);

    commands
        .spawn(WinTimer {
            win_time: time.elapsed_seconds_f64() + COUNTDOWN,
        })
        .insert(Transform {
            translation: Vec3::new(50.0, 200.0, 0.0),
            ..Default::default()
        })
        .insert(game_shape::circle::Circle {}.get_shape_bundle(
            100f32,
            DrawMode::Stroke(StrokeMode::color(Color::BLACK)), // ShapeAppearance {
                                                               //     fill: Color::Hsla {
                                                               //         hue: (100f32),
                                                               //         saturation: (70f32),
                                                               //         lightness: (70f32),
                                                               //         alpha: (0.5),
                                                               //     },
                                                               //     stroke: Color::BLACK,
                                                               //     line_width: 0f32,
                                                               // },
        ));

}

fn check_for_contacts(
    mut commands: Commands,
    win_timer: Query<(Entity, &WinTimer)>,
    mut collision_events: EventReader<CollisionEvent>,
    dragged: Query<With<Dragged>>,
    rapier_config: ResMut<RapierConfiguration>
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
        scale_time(rapier_config, 1.);
        commands.entity(win_timer.single().0).despawn();
    }
}
