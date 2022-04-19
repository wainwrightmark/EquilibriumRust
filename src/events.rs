use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RotateEvent>()
            .add_event::<DragStartEvent>()
            .add_event::<DragMoveEvent>()
            .add_event::<DragEndEvent>()
            .add_event::<DragEndedEvent>()
            .add_event::<NewGameEvent>();
    }
}

#[derive(Debug)]
pub struct RotateEvent {
    //entity: Entity,
    pub clockwise: bool, // rotation: f32,
                         // rotation_interval: f32
}

#[derive(Debug)]
pub struct DragStartEvent {
    pub drag_source: DragSource,
    pub position: Vec2,
}

#[derive(Debug)]
pub struct DragMoveEvent {
    pub drag_source: DragSource,
    pub new_position: Vec2,
}

#[derive(Debug)]
pub struct DragEndEvent {
    pub drag_source: DragSource,
}

#[derive(Debug)]
pub struct DragEndedEvent {}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DragSource {
    Mouse,
    Touch { id: u64 },
}

#[derive(Debug)]
pub struct NewGameEvent {
    pub box_count_change: i32,
}
