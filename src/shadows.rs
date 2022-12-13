use bevy::prelude::{Plugin, Query, Added, Commands, RemovedComponents, IntoSystemDescriptor, Entity, Resource, ResMut};

use crate::{components::{TouchRotate, Dragged}, draggable::*};

pub struct ShadowsPlugin{}

#[derive(Resource)]
pub struct ShowShadows(bool);

impl Plugin for ShadowsPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .insert_resource(ShowShadows(false))
        .add_system(create_shadows.after(drag_start))
        .add_system(remove_shadows.after(drag_end));
    }
}

pub fn create_shadows(
    added: Query<Added<TouchRotate>>,
    mut show_shadows: ResMut<ShowShadows>,
    // dragged: Query<(Entity, &Dragged)>,
    // commands: Commands,
){
    if added.is_empty(){return;}
    show_shadows.0 = true;
}

pub fn remove_shadows(
    removed_touch_rotates: RemovedComponents<TouchRotate>,
    touch_rotates: Query<&TouchRotate>,
    mut show_shadows: ResMut<ShowShadows>,
    // commands: Commands,
){
    if removed_touch_rotates.iter().next().is_some() && touch_rotates.is_empty(){
        show_shadows.0 = false;
    }
}