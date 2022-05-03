use bevy::input::keyboard::*;
use bevy::input::mouse::*;
use bevy::input::touch::*;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_listener.label("keyboard_listener"))
            .add_system(mousewheel_listener.label("mousewheel_listener"))
            .add_system(mousebutton_listener.label("mousebutton_listener"))
            .add_system(touch_listener.label("touch_listener"));
    }
}

fn mousebutton_listener(
    mouse_button_input: Res<Input<MouseButton>>,
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut ew_drag_start: EventWriter<DragStartEvent>,
    mut ew_drag_move: EventWriter<DragMoveEvent>,
    mut ew_drag_end: EventWriter<DragEndEvent>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        debug!("Sent mouse drag end event");
        ew_drag_end.send(DragEndEvent {
            drag_source: DragSource::Mouse,
        })
    } else if mouse_button_input.just_pressed(MouseButton::Left) {
        
        if let Some(position) = get_cursor_position(windows, q_camera) {
            debug!("Sent mouse left just pressed event {position}");
            ew_drag_start.send(DragStartEvent {
                drag_source: DragSource::Mouse,
                position,
            });
        }
    } else if mouse_button_input.pressed(MouseButton::Left) {
        
        if let Some(position) = get_cursor_position(windows, q_camera) {

            debug!("Sent mouse left is pressed event {position}");
            ew_drag_move.send(DragMoveEvent {
                drag_source: DragSource::Mouse,
                new_position: position,
            })
        }
    }
}

pub fn get_cursor_position(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };


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

        debug!("Mouse at {screen_pos} in ${window_size} / {:?},{:?} ({:?}, {:?})", wnd.physical_width(), wnd.physical_height(), wnd.scale_factor(), wnd.backend_scale_factor());

        // reduce it to a 2D value and rescale it to the world
        return Some(world_pos.truncate() );
    } else {
        return None;
    }
}

fn touch_listener(
    mut touch_evr: EventReader<TouchInput>,

    mut ew_drag_start: EventWriter<DragStartEvent>,
    mut ew_drag_move: EventWriter<DragMoveEvent>,
    mut ew_drag_end: EventWriter<DragEndEvent>,
) {

    for ev in touch_evr.iter() {

        debug!("Touch Event {:?}", ev);

        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                ew_drag_start.send(DragStartEvent {
                    drag_source: DragSource::Touch { id: ev.id },
                    position: ev.position ,
                });
                debug!("Touch {} started at: {:?}", ev.id, ev.position);
            }
            TouchPhase::Moved => {
                ew_drag_move.send(DragMoveEvent {
                    drag_source: DragSource::Touch { id: ev.id },
                    new_position: ev.position,
                });
                debug!("Touch {} moved to: {:?}", ev.id, ev.position);
            }
            TouchPhase::Ended => {
                ew_drag_end.send(DragEndEvent {
                    drag_source: DragSource::Touch { id: ev.id },
                });
                debug!("Touch {} ended at: {:?}", ev.id, ev.position);
            }
            TouchPhase::Cancelled => {
                ew_drag_end.send(DragEndEvent {
                    drag_source: DragSource::Touch { id: ev.id },
                });
                debug!("Touch {} cancelled at: {:?}", ev.id, ev.position);
            }
        }
    }
}

fn keyboard_listener(
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

fn mousewheel_listener(
    mut scroll_evr: EventReader<MouseWheel>,
    mut ev_rotate: EventWriter<RotateEvent>,
) {
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
