use bevy::prelude::*;

use crate::game_shape::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub const SHAPE_SIZE: f32 = 60f32;

pub fn create_game(mut commands: Commands, rapier_config: Res<RapierConfiguration>) {
    let physics_scale = rapier_config.scale;
    create_boxes(&mut commands, physics_scale);


    create_foundations(&mut commands, physics_scale,&GameShape::Box);
}

pub fn create_foundations(mut commands: &mut Commands,  physics_scale: f32, shape: &GameShape)
{
    let x = 0f32;
    let y = SHAPE_SIZE - (crate::WINDOW_HEIGHT / 2.0);

    create_shape(
        &mut commands,
        shape,
        SHAPE_SIZE,
        physics_scale,
        nalgebra::Vector2::<f32>::new(x,y),
        0f32,
        false,
        ShapeAppearance { fill: (Color::GRAY), ..Default::default() }
    );
}


#[derive(Component)]
pub struct Foundation{}

pub fn create_boxes(mut commands: &mut Commands, physics_scale: f32) {
    let mut rng = rand::thread_rng();

    for shape in crate::game_shape::game_shapes() {
        let rangex = -100f32..100f32;
        let rangey = -100f32..100f32;

        let point = nalgebra::Vector2::<f32>::new(rng.gen_range(rangex), rng.gen_range(rangey));

        let angle = rng.gen_range(0f32..std::f32::consts::TAU);

        create_shape(
            &mut commands,
            &shape,
            SHAPE_SIZE,
            physics_scale,
            point.into(),
            angle,
            true,
            ShapeAppearance { fill: (shape.default_fill_color()), ..Default::default() }
        );
    }
}

pub fn create_shape(
    commands: &mut Commands,
    shape: &GameShape,
    shape_size: f32,
    physics_scale: f32,
    position: nalgebra::Vector2<f32>,
    angle : f32,
    dynamic: bool,
    appearance: ShapeAppearance
) {
    let collider_shape = shape.to_collider_shape(shape_size, physics_scale);
    let position_component : Isometry<Real> = Isometry::<Real>::new(position /physics_scale, angle);

    let rbb: RigidBodyBundle = if dynamic {
        RigidBodyBundle {
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            }
            .into(),
            body_type: RigidBodyType::Dynamic.into(),
            position : position_component.into(),
            ..Default::default()
        }
    } else {
        RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position : position_component.into(),
            ..Default::default()
        }
    };

    

    let mut entity_builder = commands.spawn();
    let name = shape.name();

    entity_builder.insert_bundle(shape.get_shapebundle(shape_size, appearance))
    .insert_bundle(rbb)
        .insert_bundle(ColliderBundle {
            shape: collider_shape.into(),
            //position: position_component.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Name::new(name))
        ;

    if dynamic{
        entity_builder.insert(crate::Draggable {
            drag_mode: crate::DragMode::Release,
        });
    }
    else{
        entity_builder.insert(Foundation{});
    }
    
    //println!("Spawn {:?} {:?}", entity_builder.id(), shape);
}
