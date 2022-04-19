use bevy::prelude::*;
use bevy::log::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

pub const WINDOW_WIDTH: f32 = 360f32;
pub const WINDOW_HEIGHT: f32 = 640f32;
pub const WALL_WIDTH: f32 = 360f32;
mod draggable;
use draggable::*;

mod walls;
use walls::*;

mod shape_maker;
use shape_maker::*;

mod buttons;
use buttons::*;

mod game_shape;
use game_shape::*;

mod win;
use win::*;

mod input;
use input::*;

mod events;
use events::*;

mod components;
use components::*;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
    .insert_resource(LogSettings {
        level: Level::DEBUG,
        ..Default::default()
    })
        .insert_resource(WindowDescriptor {
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#game".to_string()),
            title: "Equilibrium".to_string(),
            width:  WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.95)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WallsPlugin)
        .add_plugin(ButtonPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup.system().label("main_setup"))
        
        .add_plugin(DragPlugin)
        .add_plugin(WinPlugin)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_game)
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(LogDiagnosticsPlugin::default());
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(bevy::core::FixedTimestep::step(10f64))
        //         .with_system(print_all_positions)
        // )
        .run();
}


fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vector2::new(0.0, -9.8);

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    rapier_config.scale = WINDOW_HEIGHT / 10.0; //The world is 10 metres tall
}

// fn print_all_positions(stuff: Query<(&Transform, &RigidBodyPositionComponent, &Name)>,){
//     for (t, c, n) in stuff.iter(){
//         let physics_position = c.position;
//         let name = n.to_string();
//         let trans = t.translation;
//         let rot = t.rotation;
//         let scale = t.scale;

//         println!("{name}: {physics_position}")
//     }
// }
