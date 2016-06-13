#[macro_use]
extern crate scad_generator;
use scad_generator::*;

pub mod rc;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
