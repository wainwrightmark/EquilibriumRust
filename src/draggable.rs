use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseWheel},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(PartialEq, Eq)]
pub enum DragMode {
    Release,
    Return,
}

#[derive(Component)]
pub struct Draggable {
    pub drag_mode: DragMode,
}

#[derive(Component)]
pub struct Dragged {
    origin: Vec2,
    offset: Vec2,
}

pub struct DragPlugin;
impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RotateEvent>()
            .add_system(mouse_press_start_drag_system)
            .add_system(mouse_release_stop_drag_system)
            .add_system(mouse_wheeel_scroll_rotate_system)
            .add_system(drag_system)
            .add_system(
                mouse_wheeel_scroll_rotate_system
                    .system()
                    .label("mouse_wheeel_scroll_rotate_system")
                    .before("handle_rotate_events"),
            ).add_system(
                keyboard_rotate_system
                    .system()
                    .label("keyboard_rotate_system")
                    .before("handle_rotate_events"),
            )
            .add_system(handle_rotate_events.system().label("handle_rotate_events"));
    }
}

pub struct RotateEvent {
    //entity: Entity,
    clockwise: bool, // rotation: f32,
                     // rotation_interval: f32
}

pub fn handle_rotate_events(
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

pub fn keyboard_rotate_system(
    mut key_evr: EventReader<KeyboardInput>,
    mut rotate_evw: EventWriter<RotateEvent>,
) {
    use bevy::input::ElementState;

    for ev in key_evr.iter() {
        if let Some(code) = ev.key_code {
            match ev.state {
                ElementState::Pressed => match code {
                    KeyCode::E => rotate_evw.send(RotateEvent { clockwise: false }),
                    KeyCode::Q => rotate_evw.send(RotateEvent { clockwise: true }),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

pub fn mouse_wheeel_scroll_rotate_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut ev_rotate: EventWriter<RotateEvent>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        let event = if ev.x + ev.y > 0f32 {
            RotateEvent { clockwise: true }
        } else {
            RotateEvent { clockwise: false }
        };

        match ev.unit {
            MouseScrollUnit::Line => {
                ev_rotate.send(event);
            }
            MouseScrollUnit::Pixel => {
                ev_rotate.send(event);
            }
        }
    }
}


pub fn mouse_release_stop_drag_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut dragged: Query<(
        Entity,
        &Draggable,
        &Dragged,
        &mut RigidBodyPositionComponent,
    )>,
    mut commands: Commands,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for mut d in dragged.iter_mut() {
        if d.1.drag_mode == DragMode::Return {
            d.3.next_position = d.2.origin.into();
        }

        
        commands
            .entity(d.0)
            .remove::<Dragged>()
            .remove::<RigidBodyTypeComponent>()
            .insert(RigidBodyTypeComponent(RigidBodyType::Dynamic));
    }
}

pub fn drag_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut dragged: Query<(&Dragged, &mut RigidBodyPositionComponent)>,
    rapier_config: Res<RapierConfiguration>,
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let scale = rapier_config.scale;

    if let Some(position) = get_cursor_position(windows, q_camera, rapier_config) {
        for mut thing in dragged.iter_mut() {
            let max_x: f32 = (crate::WINDOW_WIDTH / 2.0 ) / scale; //You can leave the box but can't go too far
            let max_y: f32 = (crate::WINDOW_HEIGHT / 2.0 ) / scale;

            let min_x: f32 = -max_x;
            let min_y: f32 = -max_y;

             let clamped_position =
                 bevy::math::Vec2::clamp(position, Vec2::new(min_x, min_y), Vec2::new(max_x, max_y));

            let new_position = thing.0.offset + clamped_position;// clamped_position;

            thing.1.next_position.translation = new_position.into();
        }
    }
}

pub fn mouse_press_start_drag_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    collider_query: QueryPipelineColliderComponentsQuery,
    draggables: Query<(With<Draggable>, &RigidBodyPositionComponent)>,
    query_pipeline: Res<QueryPipeline>,
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    if let Some(position) = get_cursor_position(windows, q_camera, rapier_config) {
        let groups = InteractionGroups::all();
        let filter = None;

        query_pipeline.intersections_with_point(
            &collider_set,
            &position.into(),
            groups,
            filter,
            |handle| {
                let entity = handle.entity();
                if let Some((_, rb)) = draggables.get(entity).ok() {
                    //println!("Entity {:?} set to dragged", entity);

                    let origin = rb.position.translation.into();
                    let offset: Vec2 = origin - position;

                    commands
                        .entity(entity)
                        .insert(Dragged {
                            origin: origin,
                            offset: offset,
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

pub fn get_cursor_position(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    rapier_config: Res<RapierConfiguration>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to
    let wnd = wnds.get(camera.window).unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value and rescale it to the world
        return Some(world_pos.truncate() / rapier_config.scale);
    } else {
        return None;
    }
}
