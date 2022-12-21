use super::GameShapeBody;
use bevy::prelude::{Transform, Vec2};
use bevy_prototype_lyon::{
    prelude::{DrawMode, GeometryBuilder},
    shapes::Polygon,
};
use bevy_rapier2d::prelude::Collider;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct PolygonBody<const SQUARES: usize, const POINTS: usize>(
    pub &'static [(isize, isize); POINTS],
);

impl<const S: usize, const P: usize> GameShapeBody for PolygonBody<S, P> {
    fn to_collider_shape(&self, shape_size: f32) -> Collider {
        let u = shape_size / (1.0 * f32::sqrt(S as f32));

        let vertices = self
            .0
            .map(|(x, y)| Vec2::new((x as f32) * u, (y as f32) * u));
        let start_indices: [[u32; 2]; P] =
            core::array::from_fn(|i| [i as u32, ((i + 1) % P) as u32]);
        Collider::convex_decomposition(&vertices, &start_indices)
    }

    fn get_shape_bundle(
        &self,
        shape_size: f32,
        draw_mode: DrawMode,
    ) -> bevy_prototype_lyon::entity::ShapeBundle {
        let u = shape_size / (1.0 * f32::sqrt(S as f32));

        //let offset = Vec2::new(u * -1., u * -1.);
        let shape = Polygon {
            points: self
                .0
                .map(|(x, y)| Vec2::new((x as f32) * u, (y as f32) * u))
                .into(),
            closed: true,
        };

        GeometryBuilder::build_as(&shape, draw_mode, Transform::default())
    }
}
