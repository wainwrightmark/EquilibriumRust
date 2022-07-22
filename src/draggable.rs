use bevy::prelude::*;
use bevy_rapier2d::prelude::*;



use crate::*;

pub struct DragPlugin;
impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                drag_start
                    .label("drag_start")
                    .after("mousebutton_listener")
                    .after("touch_listener"),
            )
            .add_system(
                drag_move
                    .label("drag_move")
                    .after("mousebutton_listener")
                    .after("touch_listener"),
            )
            .add_system(
                handle_rotate_events
                    .label("handle_rotate_events")
                    .after("mousewheel_listener")
                    .after("keyboard_listener")
                    .after("touch_listener"),
            )
            .add_system(
                drag_end
                    .label("drag_end")
                    .after("mousebutton_listener")
                    .after("touch_listener"),
            );
    }
}

fn handle_rotate_events(
    mut ev_rotate: EventReader<RotateEvent>,
    mut dragged: Query<(&mut Transform,  With<Dragged>)>,
) {
    for ev in ev_rotate.iter() {
        for (mut rb, _) in dragged.iter_mut() {
            const INTERVAL: f32 = std::f32::consts::TAU / 16.0;
            rb.rotation = rb.rotation * Quat::from_rotation_z(if ev.clockwise { INTERVAL } else { -INTERVAL });        
        }
    }
}

fn drag_end(
    mut er_drag_end: EventReader<DragEndEvent>,
    mut dragged: Query<(
        Entity,
        &Draggable,
        &Dragged,
        &mut Transform,
    )>,
    mut commands: Commands,
    mut ew_end_drag: EventWriter<DragEndedEvent>,
) {
    for event in er_drag_end.iter() {
        //println!("{:?}", event);

        dragged
            .iter_mut()
            .filter(|f| f.2.drag_source == event.drag_source)
            .for_each(|(entity, _,_,_)| {

            commands
                .entity(entity)
                .remove::<Dragged>()
                .remove::<RigidBody>()
                .insert(RigidBody::Dynamic);

            ew_end_drag.send(DragEndedEvent {});
        });
    }
}

fn drag_move(
    mut er_drag_move: EventReader<DragMoveEvent>,
    mut dragged_entities: Query<(&Dragged, &mut Transform)>
) {
    for event in er_drag_move.iter() {

        //println!("{:?}", event);

        if let Some((dragged, mut rb)) = dragged_entities
            .iter_mut()
            .filter(|d| d.0.drag_source == event.drag_source)
            .next()
        {
            //println!("Drag Move");

            let max_x: f32 = crate::WINDOW_WIDTH / 2.0 ; //You can't leave the game area
            let max_y: f32 = crate::WINDOW_HEIGHT / 2.0;

            let min_x: f32 = -max_x;
            let min_y: f32 = -max_y;

            let clamped_position = bevy::math::Vec2::clamp(
                event.new_position,
                Vec2::new(min_x, min_y),
                Vec2::new(max_x, max_y),
            );

            let new_position = dragged.offset + clamped_position.extend(0.0); // clamped_position;

            rb.translation = new_position;
        }
    }
}

fn drag_start(
    mut er_drag_start: EventReader<DragStartEvent>,
    rapier_context: Res<RapierContext>,
    draggables: Query<(With<Draggable>, &Transform)>,
    
    mut commands: Commands,
) {
    for event in er_drag_start.iter() {

        rapier_context.intersections_with_point(            
            event.position,
            default(),
            |entity| {                
                if let Some((_, rb)) = draggables.get(entity).ok() {
                    //println!("Entity {:?} set to dragged", entity);

                    let origin = rb.translation;
                    let offset = origin - event.position.extend(0.0);

                    commands
                        .entity(entity)
                        .insert(Dragged {
                            origin: origin,
                            offset: offset,
                            drag_source: event.drag_source,
                        })
                        .remove::<RigidBody>()
                        .insert(RigidBody::KinematicPositionBased);
                    return false;
                }
                return true;
            },
        );
    }
}
