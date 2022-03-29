use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    draggable::{Draggable, Dragged},
    game_shape::{GameShape, ShapeAppearance},
    shape_maker::{create_shape, SHAPE_SIZE},
    walls::Wall,
};

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewGameEvent>()
            .add_system(check_for_contacts.label("check_for_contacts"))
            .add_system(
                check_for_win
                    .label("check_for_win")
                    .after("check_for_contacts"),
            )
            .add_system(
                handle_new_game
                    .system()
                    .label("handle_new_game")
                    .after("check_for_win"),
            )
            .add_system_set(
                SystemSet::new()
                    //.with_run_criteria(bevy::core::FixedTimestep::step(0.4f64))
                    .with_system(check_for_tower.system().label("check_for_tower").after("check_for_contacts")),
            );
    }
}

#[derive(Component)]
pub struct WinTimer {
    pub win_time: f64,
}

pub struct NewGameEvent {
    pub box_count_change: i32
}



const COUNTDOWN: f64 = 3.0;

pub fn handle_new_game(
    mut commands: Commands,
    mut new_game_events: EventReader<NewGameEvent>,
    draggables: Query<(Entity, Or<(With<Draggable>, With<crate::Foundation>)> )>,
    rapier_config: Res<RapierConfiguration>,
) {
    let scale = rapier_config.scale;

    let mut first = true;
    for _ng in new_game_events.iter() {
        if !first {
            continue;
        }
        first = false;

        let mut shape_count = -1;
        for (e, _) in draggables.iter() {
    
            commands.entity(e).despawn();
            shape_count += 1;
        }
        shape_count += _ng.box_count_change; //reate one more shape for new game

        let mut rng = rand::thread_rng();

        let foundation_shape = crate::game_shape::get_random_shape(&mut rng);

        crate::create_foundations(&mut commands, scale, &foundation_shape);

        for _ in 0..shape_count {
            let shape = crate::game_shape::get_random_shape(&mut rng);

            let rangex = -100f32..100f32;
            let rangey = -100f32..100f32;

            let point = nalgebra::Vector2::<f32>::new(rng.gen_range(rangex), rng.gen_range(rangey));

            let angle = rng.gen_range(0f32..std::f32::consts::TAU);

            create_shape(
                &mut commands,
                &shape,
                SHAPE_SIZE,
                scale,
                point.into(),
                angle,
                true,
                ShapeAppearance {
                    fill: (shape.default_fill_color()),
                    ..Default::default()
                },
            );
        }
    }
}

pub fn check_for_win(
    mut commands: Commands,
    mut win_timer: Query<(Entity, &WinTimer, &mut Transform)>,
    time: Res<Time>,
    mut new_game_events: EventWriter<NewGameEvent>,
) {
    if let Some((timer_entity, timer,mut timer_transform)) = win_timer.get_single_mut().ok() {
        let remaining = timer.win_time - time.seconds_since_startup();

        if remaining <= 0f64 {
            //println!("Win - Despawn Win Timer {:?}", timer_entity);
            commands.entity(timer_entity).despawn();
            //println!("Win");
            new_game_events.send(NewGameEvent {box_count_change: 1});
        } else {
            let new_scale = (remaining / COUNTDOWN)  as f32; 

            timer_transform.scale = Vec3::new(new_scale, new_scale, 1.0);
        }
    }
}

pub fn check_for_tower(
    mut commands: Commands,
    mut end_drag_events: EventReader<crate::EndDragEvent>,
    win_timer: Query<&WinTimer>,
    time: Res<Time>,
    dragged: Query<With<Dragged>>,
    mut intersection_events:  ResMut<bevy::app::Events<IntersectionEvent>>,
    mut contact_events:  ResMut<bevy::app::Events<ContactEvent>>,
    narrow_phase: Res<NarrowPhase>,
    walls: Query<(Entity, With<Wall>)>,
) {

    if !end_drag_events.iter().any(|_|true){
        return;
    }

    //println!("Drag Ended");

    if !win_timer.is_empty() {

        //println!("Wintimer Exists");
        return; // no need to check, we're already winning
    }

    if !dragged.is_empty() {
        //println!("Something is dragged");
        return; //Something is being dragged so the player can't win yet
    }

    //Clear the events so the win timer isn't immediately despawned
    // if intersection_events.drain().any(|_| true){
    //     return;
    // }
    // if contact_events.drain().any(|_| true){
    //     return;
    // }

    //println!("Checking Contacts");

    //Check for contacts
    for (wall, _) in walls.iter() {
        for contact in narrow_phase.contacts_with(wall.handle()) {
            if contact.has_any_active_contact {
                return;
            }
        }
    }
    
    //println!("Spawning Win Timer");

    intersection_events.clear();
    contact_events.clear();

    commands
        .spawn()
        .insert(WinTimer {
            win_time: time.seconds_since_startup() + COUNTDOWN,
        })
        .insert(Transform{translation: Vec3::new(50.0, 200.0, 0.0) , ..Default::default()})
        .insert_bundle(GameShape::Circle.get_shapebundle(100f32, 
            ShapeAppearance{fill: Color::Hsla { hue: (100f32), saturation: (70f32), lightness: (70f32), alpha: (0.5) },
            stroke: Color::BLACK,
            line_width: 0f32        
        }));

    //println!("Tower Built");
}

fn check_for_contacts(
    mut commands: Commands,
    win_timer: Query<(Entity, &WinTimer)>,
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
    dragged: Query<With<Dragged>>,
    //named: Query<&Name>
) {
    if win_timer.is_empty() {
        return; // no need to check
    }

    let mut fail: Option<&str> = None;

    for _ie in intersection_events.iter() {
        //let name1 = named.get(ie.collider1.entity()).map(|x|x.to_string()).unwrap_or("unknown".to_string());
        //let name2 = named.get(ie.collider2.entity()).map(|x|x.to_string()).unwrap_or("unknown".to_string());
        //println!("Intersection Found {name1} {name2}");
        fail = Some("Intersection Found");
    }

    for _ in contact_events.iter() {
        fail = Some("Contact Found");
    }

    if fail.is_none() {
        if !dragged.is_empty() {            
            fail = Some("Something Dragged");
        }
    }

    if let Some(_error_message) = fail {
        // println!(            "{error_message} - Despawn Win Timer {:?}",
        //     win_timer.single().0
        // );
        commands.entity(win_timer.single().0).despawn();
    }
}
