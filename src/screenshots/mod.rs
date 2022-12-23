use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::*;

pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TakeScreenshotEvent>()
            .add_system_to_stage(CoreStage::Last, take_screenshots);
    }
}

pub struct TakeScreenshotEvent;

fn take_screenshots(
    mut events: EventReader<TakeScreenshotEvent>,
    query: Query<(&Transform, &Path, &DrawMode), (Without<Wall>, Without<Padlock>)>,
) {
    for _event in events.iter() {
        let svg = create_svg(query.iter());
        // println!("{svg}")
    }
}

pub fn create_svg<'a, I: Iterator<Item = (&'a Transform, &'a Path, &'a DrawMode)>>(
    iterator: I,
) -> String {
    let mut str: String = "".to_owned();

    let left = WINDOW_WIDTH * -0.5;
    let top = WINDOW_HEIGHT * -0.5;

    str.push_str(
        format!(
            r#"<svg viewbox = "{left} {top} {WINDOW_WIDTH} {WINDOW_HEIGHT}" xmlns="http://www.w3.org/2000/svg">"#
        )
        .as_str(),
    );
    str.push_str(format!(r#"<g transform=scale(1,-1);transform-origin: center;>"#).as_str());
    str.push('\n');
    for (transform, path, draw_mode) in iterator {
        let path = path.0.clone();

        let transform_svg = get_transform_svg(transform);
        str.push_str(format!(r#"<g {transform_svg}>"#).as_str());
        str.push('\n');
        let path_d = format!("{:?}", path);
        let path_style = get_path_style(draw_mode);

        str.push_str(format!(r#"<path {path_style} d={path_d} />"#).as_str());
        str.push('\n');

        str.push_str("</g>");
        str.push('\n');
    }
    str.push_str("</g>");
    str.push('\n');
    str.push_str("</svg>");

    str
}

fn get_path_style(draw_mode: &DrawMode) -> String {
    match draw_mode {
        DrawMode::Fill(fill_mode) => get_fill_style(fill_mode),
        DrawMode::Stroke(stroke_mode) => get_stroke_style(stroke_mode),
        DrawMode::Outlined {
            fill_mode,
            outline_mode,
        } => format!(
            "{} {}",
            get_fill_style(fill_mode),
            get_stroke_style(outline_mode)
        ),
    }
}

fn get_fill_style(fill_mode: &FillMode) -> String {
    format!(r#"fill = "{}""#, color_to_rgba(fill_mode.color))
}

fn get_stroke_style(stroke_mode: &StrokeMode) -> String {
    format!(r#"stroke = "{}""#, color_to_rgba(stroke_mode.color))
}

fn color_to_rgba(color: Color) -> String {
    let [mut r, mut g, mut b, mut a] = color.as_rgba_f32();
    r *= 255.0;
    g *= 255.0;
    b *= 255.0;
    a *= 255.0;
    format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        r as u8, g as u8, b as u8, a as u8
    )
}

fn get_transform_svg(transform: &Transform) -> String {
    let scale_x = transform.scale.x;
    let scale_y = transform.scale.y;

    let degrees = transform.rotation.to_axis_angle().1.to_degrees();
    let translate_x = transform.translation.x;
    let translate_y = transform.translation.y;
    format!(
        r#"transform="translate({translate_x:.1},{translate_y:.1}) rotate({degrees:.1}) scale({scale_x:.1} {scale_y:.1})""#,
    )
}
