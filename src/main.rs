use bevy::log::*;
use bevy::prelude::*;
use bevy::window::WindowResizeConstraints;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub const WINDOW_WIDTH: f32 = 360f32;
pub const WINDOW_HEIGHT: f32 = 640f32;
pub const WALL_WIDTH: f32 = 360f32;
mod draggable;
pub mod grid;
use draggable::*;

mod walls;
use walls::*;

mod shape_maker;
use shape_maker::*;

mod buttons;
use buttons::*;

mod win;
use win::*;

mod input;
use input::*;

mod events;
use events::*;

mod components;
use components::*;

pub mod game_shape;

#[cfg(target_arch = "wasm32")]
mod wasm;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let window_plugin = WindowPlugin {
        window: WindowDescriptor {
            title: "Equilibrium".to_string(),
            canvas: Some("#game".to_string()),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resize_constraints: WindowResizeConstraints {
                min_width: WINDOW_WIDTH,
                max_width: f32::MAX,
                
                min_height: WINDOW_HEIGHT,
                max_height: f32::MAX,
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let log_plugin = LogPlugin {
        level: Level::INFO,
        ..Default::default()
    };
    let mut builder = App::new();

    builder
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.95)))
        .add_plugins(DefaultPlugins.set(window_plugin).set(log_plugin))
        .add_plugin(WallsPlugin)
        .add_plugin(ButtonPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            WINDOW_HEIGHT / 10.0,
        ))
        .add_startup_system(setup)
        .add_plugin(DragPlugin)
        .add_plugin(WinPlugin)
        // .add_plugin(shadows::ShadowsPlugin{})
        .add_startup_system_to_stage(StartupStage::PostStartup, create_game);

    #[cfg(target_arch = "wasm32")]
    builder.add_plugin(wasm::WASMPlugin);

    if cfg!(debug_assertions) {
        builder.add_plugin(RapierDebugRenderPlugin::default());
    }

    //.add_plugin(FrameTimeDiagnosticsPlugin::default())
    //.add_plugin(LogDiagnosticsPlugin::default());
    builder.run();
}

pub fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0.0, -1000.0);

    commands
        .spawn(Camera2dBundle::new_with_far(1000.0))
        .insert(MainCamera);
    commands
        .spawn(new_camera(1000.0, 0.33, false))
        .insert(ZoomCamera {});
}

#[derive(Component)]
pub struct ZoomCamera {}

fn new_camera(far: f32, scale: f32, is_active: bool) -> Camera2dBundle {
    // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
    // the camera's translation by far and use a right handed coordinate system
    let projection = OrthographicProjection {
        far,
        scale,
        ..Default::default()
    };
    let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
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
            is_active,
            ..Default::default()
        },
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::None,
        },
        tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::Disabled,
    }
}
