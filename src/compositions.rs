use na::Vector3;

use scad::*;

pub fn object_at_corners(
    axis1: Vector3<f32>,
    axis2: Vector3<f32>,
    distance1: f32,
    distance2: f32,
    object: ScadObject
) -> ScadObject{
    let mut result = scad!(Union;);
    for i in [-distance1 / 2., distance1 / 2.].into_iter() {
        for j in [-distance2 / 2., distance2 / 2.].into_iter() {
            result.add_child(scad!(Translate(axis1 * *i + axis2 * *j); object.clone()));
        }
    }
    result
}
