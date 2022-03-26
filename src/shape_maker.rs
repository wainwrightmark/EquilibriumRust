use bevy::prelude::*;
use bevy_rapier2d::{prelude::*};
use nalgebra::{Point2};
use bevy_prototype_lyon::prelude::*;


pub fn create_boxes(
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
) {
    let scale = rapier_config.scale;
    
    create_box(&mut commands, scale, 50f32, Point2::new(-100f32, -100f32).into(), true);

    create_box(&mut commands, scale, 50f32, Point2::new(0f32, 0f32).into(), false);

    create_box(&mut commands, scale, 50f32, Point2::new(100f32, 100f32).into(), true);
    
}

fn create_box(
    commands: &mut Commands,
    scale: f32,
    size : f32,
    position: Point2<f32>,
     dynamic: bool
){
    let shape = shapes::Rectangle {
        extents: Vec2::new(size, size),
        origin: shapes::RectangleOrigin::Center
    };

    let collider_shape = ColliderShape::cuboid(shape.extents.x/scale/2.0, shape.extents.y/scale/2.0);

    let scaled_position = position / scale;

    println!("Created box scale:{scale} size:{size} scaled_position{scaled_position}");

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined{
                    fill_mode: FillMode::color(Color::GOLD),
                    outline_mode: StrokeMode::new(Color::TEAL, 2.0),
                },
                Transform::default(),
            )
        )
        .insert_bundle(
            if dynamic{
                RigidBodyBundle {
                    ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default() }.into(),
                    body_type: RigidBodyType::Dynamic.into(),
                    ..Default::default()
                }
            }
            else{
                RigidBodyBundle {
                    body_type: RigidBodyType::KinematicPositionBased.into(),
                    ..Default::default()
                }
            }
            
            )
        .insert_bundle(ColliderBundle {
            shape: collider_shape.into(),
            position: (scaled_position).into() ,
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(crate::Draggable{drag_mode: crate::DragMode::Release});
}



