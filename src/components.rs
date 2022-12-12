use bevy::prelude::*;

use crate::{events::DragSource, game_shape::GameShape};

#[derive(Component)]
pub struct RestartButton {}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Debug)]
pub struct Draggable {
    pub game_shape: GameShape,
}

#[derive(Component)]
pub struct Locked {}

#[derive(Component)]
pub struct Dragged {
    pub origin: Vec3,
    pub offset: Vec3,
    pub drag_source: DragSource,
    pub was_locked: bool,
}

#[derive(Component)]
pub struct WinTimer {
    pub win_time: f64,
}

// #[derive(Component)]
// pub struct Foundation {}

#[derive(Component)]
pub struct Wall {}
