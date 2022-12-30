use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::RapierContext;

use crate::{game_shape::*, shape_maker::SHAPE_SIZE, walls::*};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, display_collision_markers);
    }
}

#[derive(Component, PartialEq, Eq, Hash, Debug)]
pub struct CollisionMarker {
    pub wall_entity: Entity,
    pub other_entity: Entity,
    pub index: usize
}

fn display_collision_markers(
    mut commands: Commands,
    rapier_context: ResMut<RapierContext>,
    walls: Query<(Entity, &Transform), (With<Wall>, Without<CollisionMarker>)>,
    mut markers: Query<(Entity, &mut Transform, &CollisionMarker), Without<Wall>>,
) {
    //info!("dcm1");

    let mut markers_map = HashMap::from_iter(markers.iter_mut().map(|x| (x.2, (x.0, x.1))));

    //info!("dcm markers: {}", markers_map.len());

    for (wall_entity, wall_transform) in walls.iter() {
        for contact in rapier_context
            .contacts_with(wall_entity)
            .filter(|contact| contact.has_any_active_contacts())
        {
            let mut index = 0;

            for manifold in contact.manifolds(){
                for point in manifold.points().filter(|x|x.dist() < 0.){


                    let (other_entity, wall_local_point) = if contact.collider1() == wall_entity{
                        (contact.collider2(), point.local_p1())
                    } else{
                        (contact.collider1(), point.local_p2())
                    };

                    let cm = CollisionMarker {
                        wall_entity, other_entity, index
                    };
                    let mut new_transform = wall_transform.clone();
                    //new_transform.
                    new_transform.translation += wall_local_point.extend(0.0)* rapier_context.physics_scale()  ;
                    new_transform.translation.z = 0.0;

                    //info!("dcm shape {:?} + {:?} = {:?}", wall_transform, wall_local_point, new_transform.translation);

                    //info!("{:?}", point.dist());

                    if let Some((_, mut transform)) = markers_map.remove(&cm) {
                      //  info!("dcm updated");
                        *transform = new_transform;
                    } else {
                        //info!("dcm new");
                        let draw_mode = bevy_prototype_lyon::prelude::DrawMode::Fill(
                            bevy_prototype_lyon::prelude::FillMode::color(Color::RED),
                        );
                        commands
                            .spawn(cm)
                            .insert(Circle::get_shape_bundle(
                                &Circle,
                                SHAPE_SIZE * 0.5,
                                draw_mode,
                            ))
                            .insert(new_transform);
                    }
                    index += 1;
                }
            }

            //info!("dcm contact");
            if let Some(manifold) = contact.manifolds() .next() {


                //  info!("dcm manifold");
                if let Some(point) = manifold.points().next() {



                    //    info!("dcm point");

                    // let (other_entity, wall_local_point) = if contact.collider1() == wall_entity{
                    //     (contact.collider2(), point.local_p1())
                    // } else{
                    //     (contact.collider1(), point.local_p2())
                    // };

                    // let cm = CollisionMarker {
                    //     wall_entity, other_entity
                    // };
                    // let mut new_transform = wall_transform.clone();
                    // //new_transform.
                    // new_transform.translation += wall_local_point.extend(0.0)* rapier_context.physics_scale()  ;
                    // new_transform.translation.z = 0.0;

                    // //info!("dcm shape {:?} + {:?} = {:?}", wall_transform, wall_local_point, new_transform.translation);

                    // if let Some((_, mut transform)) = markers_map.remove(&cm) {
                    //   //  info!("dcm updated");
                    //     *transform = new_transform;
                    // } else {
                    //     //info!("dcm new");
                    //     let draw_mode = bevy_prototype_lyon::prelude::DrawMode::Fill(
                    //         bevy_prototype_lyon::prelude::FillMode::color(Color::RED),
                    //     );
                    //     commands
                    //         .spawn(cm)
                    //         .insert(Circle::get_shape_bundle(
                    //             &Circle,
                    //             SHAPE_SIZE * 0.5,
                    //             draw_mode,
                    //         ))
                    //         .insert(new_transform);
                    // }
                }
            }
        }
    }

    for (_, (entity, _)) in markers_map.iter() {
        commands.entity(*entity).despawn();
    }
}
