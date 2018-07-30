use scad_generator::*;

use na::Vector3;

pub fn cut_triangle(
    lower_length: f32,
    upper_length: f32,
    height: f32,
    depth: f32
) -> ScadObject {
    let polygon = Polygon(PolygonParameters::new(
        vec!(
            vec2(-lower_length / 2., 0.),
            vec2(lower_length / 2., 0.),
            vec2(upper_length / 2., height),
            vec2(-upper_length / 2., height)
        )
    ));

    let shape = scad!(polygon);

    scad!(LinearExtrude(LinExtrudeParams{height: depth, .. Default::default()}); shape)
}

#[cfg(test)]
mod shape_tests {
    use super::*;

    #[test]
    fn cut_triangle_test() {
        let mut sfile = ScadFile::new();

        sfile.add_object(cut_triangle(10., 5., 5., 5.));

        sfile.write_to_file("cut_triangle_test.scad".into());
    }
}
