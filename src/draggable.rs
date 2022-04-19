use crate::*;

pub struct DragPlugin;
impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                drag_start
                    .system()
                    .label("drag_start")
                    .after("mousebutton_listener")
                    .after("touch_listener"),
            )
            .add_system(
                drag_move
                    .system()
                    .label("drag_move")
                    .after("mousebutton_listener")
                    .after("touch_listener"),
            )
            .add_system(
                handle_rotate_events
                    .system()
                    .label("handle_rotate_events")
                    .after("mousewheel_listener")
                    .after("keyboard_listener")
                    .after("touch_listener"),
            )
            .add_system(
                drag_end
                    .system()
                    .label("drag_end")
                    .after("mousebutton_listener")
                    .after("touch_listener"),
            );
    }
}

fn handle_rotate_events(
    mut ev_rotate: EventReader<RotateEvent>,
    mut dragged: Query<(&mut RigidBodyPositionComponent, With<Dragged>)>,
) {
    for ev in ev_rotate.iter() {
        for (mut rb, _) in dragged.iter_mut() {
            const INTERVAL: f32 = std::f32::consts::TAU / 16.0;
            let current_turns = (rb.next_position.rotation.angle() / INTERVAL).round();
            let new_turns = current_turns + (if ev.clockwise { 1.0 } else { -1.0 });

            let new_rotation = new_turns * INTERVAL;
            rb.next_position.rotation = nalgebra::UnitComplex::new(new_rotation);

            //println!("Rotation: {new_turns} - {new_rotation}");
        }
    }
}

fn drag_end(
    mut er_drag_end: EventReader<DragEndEvent>,
    mut dragged: Query<(
        Entity,
        &Draggable,
        &Dragged,
        &mut RigidBodyPositionComponent,
    )>,
    mut commands: Commands,
    mut ew_end_drag: EventWriter<DragEndedEvent>,
) {
    for event in er_drag_end.iter() {
        //println!("{:?}", event);

        for mut d in dragged
            .iter_mut()
            .filter(|f| f.2.drag_source == event.drag_source)
        {
            //println!("Drag End");
            if d.1.drag_mode == DragMode::Return {
                d.3.next_position = d.2.origin.into();
            }

            commands
                .entity(d.0)
                .remove::<Dragged>()
                .remove::<RigidBodyTypeComponent>()
                .insert(RigidBodyTypeComponent(RigidBodyType::Dynamic));

            ew_end_drag.send(DragEndedEvent {});
        }
    }
}

fn drag_move(
    mut er_drag_move: EventReader<DragMoveEvent>,
    mut dragged_entities: Query<(&Dragged, &mut RigidBodyPositionComponent)>,
    rapier_config: Res<RapierConfiguration>,
) {
    for event in er_drag_move.iter() {
        let scale = rapier_config.scale;

        //println!("{:?}", event);

        if let Some((dragged, mut rb)) = dragged_entities
            .iter_mut()
            .filter(|d| d.0.drag_source == event.drag_source)
            .next()
        {
            //println!("Drag Move");

            let max_x: f32 = (crate::WINDOW_WIDTH / 2.0) / scale; //You can't leave the game area
            let max_y: f32 = (crate::WINDOW_HEIGHT / 2.0) / scale;

            let min_x: f32 = -max_x;
            let min_y: f32 = -max_y;

            let clamped_position = bevy::math::Vec2::clamp(
                event.new_position,
                Vec2::new(min_x, min_y),
                Vec2::new(max_x, max_y),
            );

            let new_position = dragged.offset + clamped_position; // clamped_position;

            rb.next_position.translation = new_position.into();
        }
    }
}

fn drag_start(
    mut er_drag_start: EventReader<DragStartEvent>,
    collider_query: QueryPipelineColliderComponentsQuery,
    draggables: Query<(With<Draggable>, &RigidBodyPositionComponent)>,
    query_pipeline: Res<QueryPipeline>,
    mut commands: Commands,
) {
    for event in er_drag_start.iter() {
        //println!("{:?}", event);

        let groups = InteractionGroups::all();
        let filter = None;
        let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

        query_pipeline.intersections_with_point(
            &collider_set,
            &event.position.into(),
            groups,
            filter,
            |handle| {
                let entity = handle.entity();
                if let Some((_, rb)) = draggables.get(entity).ok() {
                    //println!("Entity {:?} set to dragged", entity);

                    let origin = rb.position.translation.into();
                    let offset: Vec2 = origin - event.position;

                    commands
                        .entity(entity)
                        .insert(Dragged {
                            origin: origin,
                            offset: offset,
                            drag_source: event.drag_source,
                        })
                        .remove::<RigidBodyTypeComponent>()
                        .insert(RigidBodyTypeComponent(
                            RigidBodyType::KinematicPositionBased,
                        ));
                    return false;
                }
                return true;
            },
        );
    }
}
