use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{transform_shape, Tolerance},
    shape::Shape,
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Transform, Vector};

use super::ToShape;

impl ToShape for fj::Transform {
    fn to_shape(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Shape>, ValidationError> {
        let shape = self.shape.to_shape(config, tolerance, debug_info)?;
        let mut shape = shape.into_inner();

        let transform = transform(self);

        transform_shape(&mut shape, &transform);
        let shape = validate(shape, config)?;

        Ok(shape)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        transform(self).transform_aabb(&self.shape.bounding_volume())
    }
}

fn transform(transform: &fj::Transform) -> Transform {
    let axis = Vector::from(transform.axis).normalize();
    Transform::translation(transform.offset)
        * Transform::rotation(axis * transform.angle.rad())
}
