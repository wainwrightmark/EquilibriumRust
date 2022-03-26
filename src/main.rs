use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;
use bevy_prototype_lyon::prelude::*;

mod draggable;
use draggable::*;


mod walls;
use walls::*;

mod shape_maker;
use shape_maker::*;



fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pinball2d".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WallsPlugin)
        
        .add_plugin(ShapePlugin)
        
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())

        .add_startup_system(setup.system().label("main_setup"))

        .add_plugin(DragPlugin)

        .add_startup_system_to_stage(StartupStage::PostStartup, create_boxes)
		
		//.add_plugin(FrameTimeDiagnosticsPlugin::default())
                //.add_plugin(LogDiagnosticsPlugin::default());
		
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vector2::new(0.0, -9.8);

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)        ;

    rapier_config.scale = WINDOW_HEIGHT/10.0;    //The world is 10 metres tall
}