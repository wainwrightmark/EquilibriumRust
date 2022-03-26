use bevy::prelude::*;
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
        app.add_system(mouse_press_start_drag_system)
            .add_system(mouse_release_stop_drag_system)
            .add_system(drag_system);
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

        commands.entity(d.0)
        .remove::<Dragged>()
        .remove::<RigidBodyTypeComponent>().insert(RigidBodyTypeComponent(RigidBodyType::Dynamic))              
        ;
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

    if let Some(position) = get_cursor_position(windows, q_camera, rapier_config) {
        for mut thing in dragged.iter_mut() {
            let new_position = thing.0.offset + position;

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
                if let Some(thing) = draggables.get(entity).ok() {
                    //println!("Entity {:?} set to dragged", entity);

                    
                    let origin = thing.1.position.translation.into();
                    let offset: Vec2 = origin - position;

                    

                    commands.entity(entity).insert(Dragged {
                        origin: origin,
                        offset: offset,
                    })
                    .remove::<RigidBodyTypeComponent>()
                    .insert(RigidBodyTypeComponent(RigidBodyType::KinematicPositionBased))                                    
                    ;
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
