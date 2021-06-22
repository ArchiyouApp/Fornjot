use nalgebra::Point;

use crate::geometry::{
    aabb::Aabb,
    attributes::{BoundingVolume, Surface},
};

pub struct Difference<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B, const D: usize> BoundingVolume<D> for Difference<A, B>
where
    A: BoundingVolume<D>,
{
    fn aabb(&self) -> Aabb<D> {
        // Since `self.b` is subtracted from `self.a`, the bounding volume of
        // the difference is not going to be bigger than that of `self.a`. Just
        // taking the bounding volume from `self.a` is certainly not optimal,
        // but good enough for now.
        self.a.aabb()
    }
}

impl<A, B, const D: usize> Surface<D> for Difference<A, B>
where
    A: Surface<D>,
    B: Surface<D>,
{
    fn surface(&self, point: impl Into<Point<f32, D>>) -> f32 {
        let point = point.into();

        let dist_a = self.a.surface(point);
        let dist_b = self.b.surface(point);

        if dist_a > -dist_b {
            dist_a
        } else {
            -dist_b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{attributes::Surface, shapes::Sphere};

    use super::Difference;

    #[test]
    fn distance() {
        let difference = Difference {
            a: Sphere::new().with_radius(1.0),
            b: Sphere::new().with_radius(0.5),
        };

        assert_eq!(difference.surface([0.0, 0.0, 0.0]), 0.5);
        assert_eq!(difference.surface([0.5, 0.0, 0.0]), 0.0);
        assert_eq!(difference.surface([0.625, 0.0, 0.0]), -0.125);
        assert_eq!(difference.surface([0.75, 0.0, 0.0]), -0.25);
        assert_eq!(difference.surface([0.875, 0.0, 0.0]), -0.125);
        assert_eq!(difference.surface([1.0, 0.0, 0.0]), 0.0);
        assert_eq!(difference.surface([1.5, 0.0, 0.0]), 0.5);
    }
}
