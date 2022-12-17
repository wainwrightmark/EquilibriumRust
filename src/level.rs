use std::time::Duration;

use crate::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;

pub const SMALL_TEXT_COLOR: Color = Color::DARK_GRAY;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .add_startup_system(setup_level_text);
    }
}

pub fn handle_change_level(
    mut commands: Commands,
    mut change_level_events: EventReader<ChangeLevelEvent>,
    draggables: Query<(Entity, With<Draggable>)>,
    mut current_level: ResMut<CurrentLevel>,
    input_detector : Res<InputDetector>,
    level_text: Query<(Entity,  &mut Text), With<LevelText>>,
) {
    if let Some(event) = change_level_events.iter().next() {
        for (e, _) in draggables.iter() {
            commands.entity(e).despawn();
        }

        match event {
            ChangeLevelEvent::Next => current_level.0 += 1,
            ChangeLevelEvent::Previous => {
                current_level.0 = current_level.0.saturating_sub(1).max(1)
            }
            ChangeLevelEvent::Restart => (),
        }

        let level = GameLevel::get_level(current_level.0, input_detector);

        level::start_level(commands, level, level_text);
    }
}

fn start_level(
    mut commands: Commands,
    level: GameLevel,
    mut level_text: Query<(Entity,  &mut Text), With<LevelText>>,
) {
    for (entity, mut text) in level_text.iter_mut() {
        let new_text = format!("{: ^36}", level.message);

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


fn setup_level_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                flex_grow: 0.,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|f| {
            f.spawn(
                TextBundle::from_sections([TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: SMALL_TEXT_COLOR,
                })]) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::CENTER),
            )
            .insert(LevelText);
        });
}

#[derive(Component)]
pub struct LevelText;

#[derive(Default, Resource)]
pub struct CurrentLevel(pub usize);

pub struct GameLevel {
    pub message: &'static str,
    pub shapes: usize,
}

impl GameLevel {
    pub fn get_level(i: usize, input_detector: Res<InputDetector>) -> GameLevel {
        match i {
            1 => GameLevel {
                message: "move the shape",
                shapes: i,
            },
            2 => GameLevel {
                message: "build a tower with all the shapes",
                shapes: i,
            },
            3 => GameLevel {
                message: "the locked shape can be unlocked",
                shapes: i,
            },
            4 =>
            {
                let message = if input_detector.is_touch{
                    "Rotate with your finger"
                }   
                else{
                    "Rotate with the mousewheel, or Q/E"
                };

                GameLevel {
                    message,
                    shapes: i,
                }
            }
            
            

            _ => Self::generic_level(i),
        }
    }

    fn generic_level(i: usize) -> GameLevel {
        GameLevel {
            message: "",
            shapes: i,
        }
    }
}
