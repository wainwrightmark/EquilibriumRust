use bevy::prelude::*;

use crate::events::DragSource;

#[derive(Component)]
pub struct RestartButton {}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(PartialEq, Eq)]
pub enum DragMode {
    Release,
    Return,
}

#[derive(Component)]
pub struct Draggable {
    pub drag_mode: DragMode,
}

#[derive(Component)]
pub struct Dragged {
    pub origin: Vec3,
    pub offset: Vec3,
    pub drag_source: DragSource,
}

#[derive(Component)]
pub struct WinTimer {
    pub win_time: f64,
}

#[derive(Component)]
pub struct Foundation {}

#[derive(Component)]
pub struct Wall {}
