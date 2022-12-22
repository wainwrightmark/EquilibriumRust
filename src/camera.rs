use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::new_with_far(1000.0))
        .insert(MainCamera);
    // commands
    //     .spawn(new_camera(1000.0, 0.33, false))
    //     .insert(ZoomCamera {});
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ZoomCamera {pub touch_id: u64}

pub fn new_camera(far: f32, scale: f32,mut transform: Transform) -> Camera2dBundle {
    // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
    // the camera's translation by far and use a right handed coordinate system
    let projection = OrthographicProjection {
        far,
        scale,
        ..Default::default()
    };

    transform.rotation = Default::default();
    transform.translation *= 1. - scale;
    transform.translation.z = far - 0.1;


    //origin.extend(0.0) *
    //let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
    let view_projection =
        bevy::render::camera::CameraProjection::get_projection_matrix(&projection)
            * transform.compute_matrix().inverse();
    let frustum = bevy::render::primitives::Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        bevy::render::camera::CameraProjection::far(&projection),
    );
    Camera2dBundle {
        camera_render_graph: bevy::render::camera::CameraRenderGraph::new(
            bevy::core_pipeline::core_2d::graph::NAME,
        ),
        projection,
        visible_entities: bevy::render::view::VisibleEntities::default(),
        frustum,
        transform,
        global_transform: Default::default(),
        camera: Camera {
            priority: 1,
            is_active: true,
            ..Default::default()
        },
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::None,
        },
        tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::Disabled,
    }
}
