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
        )
        .add_system(add_padlock)
        .add_system_to_stage(CoreStage::PostUpdate, remove_padlock)
        .add_event::<RotateEvent>()
        .add_event::<DragStartEvent>()
        .add_event::<DragMoveEvent>()
        .add_event::<DragEndEvent>()
        .add_event::<DragEndedEvent>();
    }
}

fn handle_rotate_events(
    mut ev_rotate: EventReader<RotateEvent>,
    mut dragged: Query<(&mut Transform, With<Dragged>)>,
) {
    for ev in ev_rotate.iter() {
        for (mut rb, _) in dragged.iter_mut() {
            rb.rotation *= Quat::from_rotation_z(ev.angle);
            if let Some(multiple) = ev.snap_resolution {
                rb.rotation = round_z(rb.rotation, multiple);
            }
        }
    }
}

fn round_z(q: Quat, multiple: f32) -> Quat {
    let multiple = multiple / 2.;
    let [x, y, z, w] = q.to_array();
    let mut asin_z = z.asin();
    let mut acos_w = w.acos();
    asin_z = f32::round(asin_z / multiple) * multiple;
    acos_w = f32::round(acos_w / multiple) * multiple;

    Quat::from_xyzw(x, y, asin_z.sin(), acos_w.cos())
}

pub fn add_padlock(mut commands: Commands, locked: Query<&Transform, Added<Locked>>) {
    for transform in locked.iter() {
        let svg_doc_size = Vec2::new(512., 512.);

        let transform = Transform {
            rotation: Default::default(),
            scale: Vec3::new(0.05, 0.05, 1.),
            translation: Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z + 1.0,
            ),
        };
        commands
            .spawn(GeometryBuilder::build_as(
                &shapes::SvgPathShape {
                    svg_path_string: PADLOCK_OUTLINE.to_owned(),
                    svg_doc_size_in_px: svg_doc_size.to_owned(),
                },
                DrawMode::Fill(FillMode {
                    options: FillOptions::DEFAULT,
                    color: Color::BLACK,
                }), // ::Stroke(StrokeMode::new(Color::BLACK, 4.0)),
                transform,
            ))
            .insert(Padlock {});
    }
}

pub fn remove_padlock(
    mut commands: Commands,
    padlock: Query<Entity, With<Padlock>>,
    unlocked: RemovedComponents<Locked>,
) {
    if unlocked.iter().next().is_none() {
        return;
    }
    for entity in padlock.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn drag_end(
    mut er_drag_end: EventReader<DragEndEvent>,
    mut dragged: Query<(Entity, &Draggable, &Dragged, &mut Transform)>,
    touch_rotate: Query<(Entity, &TouchRotate)>,
    locked: Query<&Locked>,
    mut commands: Commands,
    mut ew_end_drag: EventWriter<DragEndedEvent>,
    mut cameras: Query<&mut bevy::render::camera::Camera, With<ZoomCamera>>,
) {
    for event in er_drag_end.iter() {
        debug!("{:?}", event);

        let mut any_locked = !locked.is_empty();
        let mut count = dragged.iter().count();
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
                        // .insert(DrawMode::Fill(FillMode::color(Color::GRAY)))
                        ;
                    any_locked = true;
                }
                count -= 1;
                ew_end_drag.send(DragEndedEvent {});
            });

        if let DragSource::Touch { id } = event.drag_source {
            touch_rotate
                .iter()
                .filter(|x| x.1.touch_id == id)
                .for_each(|(e, _)| commands.entity(e).despawn());

            if count == 0 {
                for mut camera in cameras.iter_mut() {
                    camera.is_active = false;
                }
            }
        };
    }
}

pub fn drag_move(
    mut er_drag_move: EventReader<DragMoveEvent>,
    mut dragged_entities: Query<(&Dragged, &mut Transform), Without<ZoomCamera>>,
    mut touch_rotate: Query<&mut TouchRotate>,
    mut ev_rotate: EventWriter<RotateEvent>,
    mut cameras: Query<(&mut Transform, &OrthographicProjection), With<ZoomCamera>>,
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

            let new_position = (dragged.offset + clamped_position).extend(0.0);

            rb.translation = new_position;
            if dragged.drag_source.is_touch() {
                for (mut camera_transform, camera) in cameras.iter_mut() {
                    camera_transform.translation = new_position * (1. - camera.scale);
                }
            }
        } else if let DragSource::Touch { id } = event.drag_source {
            if let Some(mut rotate) = touch_rotate.iter_mut().find(|x| x.touch_id == id) {
                let previous_angle = rotate.centre.angle_between(rotate.previous);
                let new_angle = rotate.centre.angle_between(event.new_position);
                rotate.previous = event.new_position;
                let angle = new_angle - previous_angle;

                ev_rotate.send(RotateEvent {
                    angle,
                    snap_resolution: None,
                })
            }
        }
    }
}

pub fn drag_start(
    mut er_drag_start: EventReader<DragStartEvent>,
    rapier_context: Res<RapierContext>,
    draggables: Query<(&Draggable, Option<&Locked>, &Transform), Without<Dragged>>,
    dragged: Query<(&Dragged, &Transform)>,
    mut cameras: Query<&mut bevy::render::camera::Camera, With<ZoomCamera>>,

    mut commands: Commands,
) {
    let mut found_touch = false;
    for event in er_drag_start.iter() {
        debug!("Drag Started {:?}", event);
        let mut found = false;
        if event.drag_source.is_touch() {
            found_touch = true;
        } else if found_touch {
            continue;
        }

        if dragged.is_empty() {
            rapier_context.intersections_with_point(event.position, default(), |entity| {
                if let Ok((draggable, locked, rb)) = draggables.get(entity) {
                    debug!("{:?} found intersection with {:?}", event, draggable);
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
                            // .insert(DrawMode::Fill(FillMode::color(
                            //     draggable.game_shape.default_fill_color(),
                            // )))
                            ;
                    }

                    if event.drag_source.is_touch() {
                        for mut camera in cameras.iter_mut() {
                            camera.is_active = true;
                        }
                    }

                    found = true;
                    return false; //Stop looking for intersections
                }
                true //keep looking for intersections
            });
        }

        if !found {
            if let DragSource::Touch { id } = event.drag_source {
                if let Some((_, transform)) = dragged
                    .iter().find(|x| matches!(x.0.drag_source, DragSource::Touch { id: _ }))
                {
                    commands.spawn(TouchRotate {
                        previous: event.position,
                        centre: transform.translation.truncate(),
                        touch_id: id,
                    });
                }
            }
        }
    }
}

const PADLOCK_OUTLINE: &str = "M254.28 17.313c-81.048 0-146.624 65.484-146.624 146.406V236h49.594v-69.094c0-53.658 43.47-97.187 97.03-97.187 53.563 0 97.032 44.744 97.032 97.186V236h49.594v-72.28c0-78.856-65.717-146.407-146.625-146.407zM85.157 254.688c-14.61 22.827-22.844 49.148-22.844 76.78 0 88.358 84.97 161.5 191.97 161.5 106.998 0 191.968-73.142 191.968-161.5 0-27.635-8.26-53.95-22.875-76.78H85.155zM254 278.625c22.34 0 40.875 17.94 40.875 40.28 0 16.756-10.6 31.23-25.125 37.376l32.72 98.126h-96.376l32.125-98.125c-14.526-6.145-24.532-20.62-24.532-37.374 0-22.338 17.972-40.28 40.312-40.28z";

#[derive(Component, Debug)]
pub struct Draggable;

#[derive(Component, Debug)]
pub struct Locked;

#[derive(Component, Debug)]
pub struct Padlock;

#[derive(Component)]
pub struct Dragged {
    pub origin: Vec2,
    pub offset: Vec2,
    pub drag_source: DragSource,
    pub was_locked: bool,
}

#[derive(Component)]
pub struct TouchRotate {
    pub previous: Vec2,
    pub centre: Vec2,
    pub touch_id: u64,
}
#[derive(Debug)]
pub struct RotateEvent {
    pub angle: f32,
    pub snap_resolution: Option<f32>,
}

#[derive(Debug)]
pub struct DragStartEvent {
    pub drag_source: DragSource,
    pub position: Vec2,
}

#[derive(Debug)]
pub struct DragMoveEvent {
    pub drag_source: DragSource,
    pub new_position: Vec2,
}

#[derive(Debug)]
pub struct DragEndEvent {
    pub drag_source: DragSource,
}

#[derive(Debug)]
pub struct DragEndedEvent {}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DragSource {
    Mouse,
    Touch { id: u64 },
}

impl DragSource {
    pub fn is_touch(&self) -> bool {
        matches!(self, DragSource::Touch { id: _ })
    }
}
