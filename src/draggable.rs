use crate::*;
use bevy_prototype_lyon::prelude::FillMode;

pub struct DragPlugin;
impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            drag_start
                .after(input::mousebutton_listener)
                .after(input::touch_listener),
        )
        .add_system(
            drag_move
                .after(input::mousebutton_listener)
                .after(input::touch_listener),
        )
        .add_system(
            handle_rotate_events
                .after(input::keyboard_listener)
                .after(input::mousewheel_listener),
        )
        .add_system(
            drag_end
                .after(input::mousebutton_listener)
                .after(input::touch_listener),
        );
    }
}

fn handle_rotate_events(
    mut ev_rotate: EventReader<RotateEvent>,
    mut dragged: Query<(&mut Transform, With<Dragged>)>,
) {
    for ev in ev_rotate.iter() {
        for (mut rb, _) in dragged.iter_mut() {
            rb.rotation *= Quat::from_rotation_z(ev.angle);
        }
    }
}

pub fn drag_end(
    mut er_drag_end: EventReader<DragEndEvent>,
    mut dragged: Query<(Entity, &Draggable, &Dragged, &mut Transform)>,
    touch_rotate: Query<(Entity, &TouchRotate)>,
    locked: Query<&Locked>,
    mut commands: Commands,
    mut ew_end_drag: EventWriter<DragEndedEvent>,
) {
    for event in er_drag_end.iter() {
        debug!("{:?}", event);

        let mut any_locked = !locked.is_empty();
        dragged
            .iter_mut()
            .filter(|f| f.2.drag_source == event.drag_source)
            .for_each(|(entity, _, dragged, _)| {
                if any_locked || dragged.was_locked {
                    commands
                        .entity(entity)
                        .remove::<Dragged>()
                        .remove::<RigidBody>()
                        .insert(RigidBody::Dynamic);
                } else {
                    commands
                        .entity(entity)
                        .remove::<Dragged>()
                        .remove::<RigidBody>()
                        .insert(RigidBody::Fixed)
                        .insert(Locked {})
                        .insert(DrawMode::Fill(FillMode::color(Color::GRAY)));
                    any_locked = true;
                }

                ew_end_drag.send(DragEndedEvent {});
            });

        if let DragSource::Touch { id } = event.drag_source {
            touch_rotate
                .iter()
                .filter(|x| x.1.touch_id == id)
                .for_each(|(e, _)| commands.entity(e).despawn());
        };
    }
}

pub fn drag_move(
    mut er_drag_move: EventReader<DragMoveEvent>,
    mut dragged_entities: Query<(&Dragged, &mut Transform)>,
    mut touch_rotate: Query<&mut TouchRotate>,
    mut ev_rotate: EventWriter<RotateEvent>,
) {
    for event in er_drag_move.iter() {
        debug!("{:?}", event);
        if let Some((dragged, mut rb)) = dragged_entities
            .iter_mut()
            .find(|d| d.0.drag_source == event.drag_source)
        {
            //debug!("Drag Move");

            let max_x: f32 = crate::WINDOW_WIDTH / 2.0; //You can't leave the game area
            let max_y: f32 = crate::WINDOW_HEIGHT / 2.0;

            let min_x: f32 = -max_x;
            let min_y: f32 = -max_y;

            let clamped_position = bevy::math::Vec2::clamp(
                event.new_position,
                Vec2::new(min_x, min_y),
                Vec2::new(max_x, max_y),
            );

            let new_position = dragged.offset + clamped_position; // clamped_position;

            rb.translation = new_position.extend(0.0);
        } else if let DragSource::Touch { id } = event.drag_source {
            if let Some(mut rotate) = touch_rotate.iter_mut().filter(|x|x.touch_id == id).next(){
                let previous_angle = rotate.centre.angle_between(rotate.previous);
                let new_angle = rotate.centre.angle_between(event.new_position);
                rotate.previous = event.new_position;
                let angle = new_angle - previous_angle;

                ev_rotate.send(RotateEvent{angle})
                
            }
        }
    }
}

pub fn drag_start(
    mut er_drag_start: EventReader<DragStartEvent>,
    rapier_context: Res<RapierContext>,
    draggables: Query<(&Draggable, Option<&Locked>, &Transform)>,
    dragged: Query<(&Dragged, &Transform)>,

    mut commands: Commands,
) {
    for event in er_drag_start.iter() {
        debug!("Drag Started {:?}", event);
        let mut found = false;
        rapier_context.intersections_with_point(event.position, default(), |entity| {
            if let Ok((draggable, locked, rb)) = draggables.get(entity) {
                debug!("Found intersection with {:?}", draggable);
                //println!("Entity {:?} set to dragged", entity);

                let origin = rb.translation.truncate();
                let offset = origin - event.position;
                let was_locked = locked.is_some();

                commands
                    .entity(entity)
                    .insert(Dragged {
                        origin,
                        offset,
                        drag_source: event.drag_source,
                        was_locked,
                    })
                    .remove::<RigidBody>()
                    .insert(RigidBody::KinematicPositionBased);

                if was_locked {
                    commands
                        .entity(entity)
                        .remove::<Locked>()
                        .insert(DrawMode::Fill(FillMode::color(
                            draggable.game_shape.default_fill_color(),
                        )));
                }
                found = true;
                return false; //Stop looking for intersections
            }
            true //keep looking for intersections
        });

        if!found{
            if let DragSource::Touch { id } = event.drag_source{
                if let Some((_, transform)) = dragged.iter().filter(|x| matches!(x.0.drag_source, DragSource::Touch { id:_ }) ).next(){
                    commands.spawn(TouchRotate{
                        previous: event.position,
                        centre: transform.translation.truncate(),
                        touch_id: id
                    });
                }

            }
        }
    }
}
