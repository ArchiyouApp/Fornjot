use std::collections::BTreeSet;

use fj_math::Point;

use crate::{
    local::Local,
    objects::{Edge, GlobalVertex, Solid, Vertex, VerticesOfEdge},
};

use super::intersection::{self, CurveFaceIntersectionList};

/// Computes the shape that is the union of the two provided shapes
pub fn union(a: Solid, b: Solid) -> Solid {
    // TASK: Implement algorithm from "Boundary Representation Modelling
    //       Techniques", section 6.1.1 (pages 127 ff.).

    let mut faces = BTreeSet::new();

    // Check the faces of both shapes for intersections.
    for face_a in a.faces() {
        for face_b in b.faces() {
            let intersection = intersection::surface_surface(
                face_a.surface(),
                face_b.surface(),
            );

            let (curve_a, curve_b, curve) = match intersection {
                Some(intersection) => intersection,
                None => {
                    // TASK: Implement.
                    continue;
                }
            };

            let intersections_a =
                CurveFaceIntersectionList::compute(&curve_a, face_a);
            let intersections_b =
                CurveFaceIntersectionList::compute(&curve_b, face_b);

            match (intersections_a.is_empty(), intersections_b.is_empty()) {
                (false, true) => {
                    faces.insert(face_a.clone());
                }
                (true, false) => {
                    faces.insert(face_b.clone());
                }
                (true, true) => {
                    faces.insert(face_a.clone());
                    faces.insert(face_b.clone());
                }
                _ => {
                    // TASK: Implement.
                    todo!()
                }
            }

            let intersections = intersections_a.merge(&intersections_b);

            for interval in intersections {
                let [start, end] = interval.map(|coord| Point::from([coord]));
                let [start_global, end_global] = [start, end].map(|point| {
                    let position = curve.point_from_curve_coords(point);
                    GlobalVertex::from_position(position)
                });

                let vertices = VerticesOfEdge::from_vertices([
                    Vertex::new(start, start_global),
                    Vertex::new(end, end_global),
                ]);

                let edge_a = Edge::new(Local::new(curve_a, curve), vertices);
                let edge_b = Edge::new(Local::new(curve_b, curve), vertices);

                // TASK: Implement.
                let _ = edge_a;
                let _ = edge_b;
            }

            // TASK: Implement.
            let _ = curve;
        }
    }

    Solid::from_faces(faces)
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::{union, TransformObject},
        objects::Solid,
    };

    #[test]
    fn distinct() {
        let a = Solid::cube_from_edge_length(1.).translate([-1., -1., -1.]);
        let b = Solid::cube_from_edge_length(1.).translate([1., 1., 1.]);

        let mut all_faces = Vec::new();
        all_faces.extend(a.faces().cloned());
        all_faces.extend(b.faces().cloned());

        let union = union(a, b);

        assert_eq!(union, Solid::from_faces(all_faces));
    }

    #[test]
    fn a_contains_b() {
        let a = Solid::cube_from_edge_length(2.);
        let b = Solid::cube_from_edge_length(1.);

        let union = union(a.clone(), b);

        assert_eq!(union, a);
    }

    #[test]
    fn b_contains_a() {
        let a = Solid::cube_from_edge_length(1.);
        let b = Solid::cube_from_edge_length(2.);

        let union = union(a, b.clone());

        assert_eq!(union, b);
    }

    // TASK: intersecting, broken edges in a

    // #[test]
    // fn intersecting_with_broken_edges_in_b() {
    //     let a = Solid::cube_from_edge_length(2.);
    //     let b = Solid::cube_from_edge_length(1.).translate([0., 0., 1.]);

    //     let union = union(a, b);

    //     let expected = todo!();
    //     assert_eq!(union, expected);
    // }
}
