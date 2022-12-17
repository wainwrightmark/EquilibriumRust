use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RotateEvent>()
            .add_event::<DragStartEvent>()
            .add_event::<DragMoveEvent>()
            .add_event::<DragEndEvent>()
            .add_event::<DragEndedEvent>()
            .add_event::<ChangeLevelEvent>();
    }
}

#[derive(Debug)]
pub struct RotateEvent {
    //entity: Entity,
    pub angle: f32, //pub clockwise: bool, // rotation: f32,
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

impl DragSource {
    pub fn is_touch(&self) -> bool {
        matches!(self, DragSource::Touch { id: _ })
    }
}

#[derive(Debug)]
pub enum ChangeLevelEvent {
    Next,
    Previous,
    Restart
}


