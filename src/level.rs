use std::time::Duration;

use crate::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;

pub const SMALL_TEXT_COLOR: Color = Color::DARK_GRAY;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .add_startup_system(setup_level_text)
            .add_event::<ChangeLevelEvent>();
    }
}

pub fn handle_change_level(
    mut commands: Commands,
    mut change_level_events: EventReader<ChangeLevelEvent>,
    draggables: Query<(Entity, With<Draggable>)>,
    mut current_level: ResMut<CurrentLevel>,
    input_detector: Res<InputDetector>,
    level_text: Query<(Entity, &mut Text), With<LevelText>>,
) {
    if let Some(event) = change_level_events.iter().next() {
        for (e, _) in draggables.iter() {
            commands.entity(e).despawn();
        }

        current_level.0 = event.apply(&current_level.0);

        level::start_level(commands, current_level.0, level_text, input_detector);
    }
}

fn start_level(
    mut commands: Commands,
    level: GameLevel,
    mut level_text: Query<(Entity, &mut Text), With<LevelText>>,
    input_detector: Res<InputDetector>,
) {
    if let Some((entity, mut text)) = level_text.iter_mut().next() {
        let new_text = format!(
            "{: ^36}",
            level.get_text(input_detector).unwrap_or_default()
        );

        text.sections[0].value = new_text;
        commands.entity(entity).insert(Animator::new(Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(10),
            TextColorLens {
                section: 0,
                start: SMALL_TEXT_COLOR,
                end: Color::NONE,
            },
        )));
    }

    shape_maker::create_level_shapes(&mut commands, level);
}

pub fn setup_level_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: SMALL_TEXT_COLOR,
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER)
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    }),
                )
                .insert(LevelText);
        });
}

#[derive(Component)]
pub struct LevelText;

#[derive(Default, Resource)]
pub struct CurrentLevel(pub GameLevel);

#[derive(Debug, Clone, Copy)]
pub struct GameLevel {
    //pub message: &'static str,
    pub shapes: usize,
    pub level_type: LevelType,
}
impl Default for GameLevel {
    fn default() -> Self {
        Self {
            shapes: 1,
            level_type: LevelType::Tutorial,
        }
    }
}

impl GameLevel {
    pub fn get_text(&self, input_detector: Res<InputDetector>) -> Option<&'static str> {
        match self.level_type {
            LevelType::Tutorial => match self.shapes {
                1 => Some("move the shape"),
                2 => Some("build a tower with all the shapes"),
                3 => Some("the locked shape can be unlocked"),
                4 => {
                    if input_detector.is_touch {
                        Some("Rotate with your finger")
                    } else {
                        Some("Rotate with the mousewheel, or Q/E")
                    }
                }
                _ => None,
            },
            LevelType::Infinite => None,
            LevelType::Challenge => Some("Daily Challenge"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LevelType {
    Tutorial,
    Infinite,
    Challenge,
}

#[derive(Debug)]
pub enum ChangeLevelEvent {
    Next,
    // Previous,
    ResetLevel,
    StartTutorial,
    StartInfinite,
    StartChallenge,
}

impl ChangeLevelEvent {
    #[must_use]
    pub fn apply(&self, level: &GameLevel) -> GameLevel {
        match self {
            ChangeLevelEvent::Next => {
                let level_type = match level.level_type {
                    LevelType::Tutorial => {
                        if level.shapes > 4 {
                            LevelType::Infinite
                        } else {
                            LevelType::Tutorial
                        }
                    }
                    LevelType::Infinite => LevelType::Infinite,
                    LevelType::Challenge => LevelType::Infinite,
                };

                GameLevel {
                    shapes: level.shapes + 1,
                    level_type,
                }
            }
            // ChangeLevelEvent::Previous => GameLevel {
            //     shapes: level.shapes.saturating_sub(1).max(1),
            //     level_type: level.level_type,
            // },
            ChangeLevelEvent::ResetLevel => level.clone(),
            ChangeLevelEvent::StartTutorial => GameLevel {
                shapes: 1,
                level_type: LevelType::Tutorial,
            },
            ChangeLevelEvent::StartInfinite => GameLevel {
                shapes: 5,
                level_type: LevelType::Infinite,
            },
            ChangeLevelEvent::StartChallenge => GameLevel {
                shapes: 10,
                level_type: LevelType::Challenge,
            },
        }
    }
}
