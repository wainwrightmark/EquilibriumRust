use super::GameShapeBody;
use crate::{grid::prelude::{Shape, PolyominoShape}, shape_appearance::*};
use bevy::prelude::{Transform, Vec2};
use bevy_prototype_lyon::{prelude::GeometryBuilder, shapes::Polygon};
use bevy_rapier2d::prelude::{Collider,  };
use itertools::Itertools;



fn get_vertices<const S : usize>(shape: &Shape<S>, shape_size: f32) -> impl Iterator<Item = Vec2> {
    let u = shape_size / (1.0 * f32::sqrt(S as f32));
    let (x_offset, y_offset) = shape.get_centre();

    shape.draw_outline().map(move |qr| {
        Vec2::new(
            ((qr.x() as f32) - x_offset) * u,
            ((qr.y() as f32) - y_offset) * u,
        )
    })
}
impl<const S: usize> GameShapeBody for Shape<S> {
    fn to_collider_shape(&self, shape_size: f32) -> Collider {
        let u = shape_size / (1.0 * f32::sqrt(S as f32));        
        let (x_offset, y_offset2) = self.get_centre();

        let shapes = self.into_iter().map(|qr|
            {
                let vect = Vec2::new(
                    ((qr.x() as f32) - x_offset + 0.5 ) * u,
                    ((qr.y() as f32) - y_offset2 + 0.5 ) * u,
                );

                (vect, 0.0, Collider::cuboid(u * 0.5, u * 0.5) )
            }
        ).collect_vec();
        
        Collider::compound(shapes)
    }

    fn get_shape_bundle(
        &self,
        shape_size: f32,
        appearance: ShapeAppearance,
    ) -> bevy_prototype_lyon::entity::ShapeBundle {
        let points = get_vertices(self, shape_size).collect_vec();
        let shape = Polygon {
            points: points,
            closed: true,
        };

        GeometryBuilder::build_as(&shape, appearance.into(), Transform::default())
    }
}
