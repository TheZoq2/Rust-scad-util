#[macro_use]
extern crate scad_generator;
use scad_generator::*;

extern crate nalgebra as na;

pub mod radio_control;
pub mod threads;
pub mod electronics;
pub mod compositions;
pub mod constants;

//Unit size nut
fn base_nut() -> ScadObject
{
    let cube_len = 1.0/2.0;
    let cube_width = 30.0_f32.to_radians().sin() / 30.0_f32.to_radians().cos();

    let mut result = scad!(Union);

    for i in 0..6
    {
        result.add_child(
            scad!(Rotate(60.0 * i as f32 + 30.0, vec3(0.0, 0.0, 1.0));
            {
                scad!(Translate(vec3(-cube_width / 2.0, 0.0, 0.0)); scad!(Cube(vec3(cube_width, cube_len, 1.0))))
            })
        );
    }

    result
}

pub fn nut(width: f32, height: f32) -> ScadObject
{
    scad!(Scale(vec3(width, width, height));
    {
        base_nut()
    })
}


pub fn add_color(color: na::Vector3<f32>, object: ScadObject) -> ScadObject
{
    scad!(Color(color); object)
}
pub fn add_named_color(color: &str, object: ScadObject) -> ScadObject
{
    scad!(NamedColor(String::from(color)); object)
}

#[cfg(test)]
mod tests {
    use scad_generator::*;

    use super::*;

    #[test]
    fn color_tests()
    {
        assert_eq!(
                add_color(na::zero(),scad!(Union)).get_code(),
                String::from("color([0,0,0])\n{\n\tunion();\n}")
            );
        assert_eq!(
                add_named_color("blue", scad!(Union)).get_code(),
                String::from("color(\"blue\")\n{\n\tunion();\n}")
            );
    }
}
