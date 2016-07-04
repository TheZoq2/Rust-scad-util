#[macro_use]
extern crate scad_generator;
use scad_generator::*;

pub mod radio_control;

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

pub fn wing_nut(nut_width: f32, center_height: f32) -> ScadObject
{
    let wing_radius = nut_width * 1.0;
    let center_diameter = nut_width * 1.5;
    let wing_thickness = nut_width * 0.2;

    let center_piece = scad!(Difference;
    {
        scad!(Cylinder(center_height, Diameter(center_diameter))),
        nut(nut_width, center_height*2.0)
    });

    let wing = scad!(Difference;
    {
        scad!(Translate(vec3(center_diameter / 2.0, -wing_thickness/2.0, 0.0));
        {
            scad!(Rotate(-45.0, vec3(0.0, 1.0, 0.0));
            {
                scad!(Cube(vec3(wing_radius, wing_thickness, center_height)))
            })
        }),
        scad!(Cylinder(center_height*2.0, Diameter(center_diameter))),
    });

    scad!(Difference;
    {
        scad!(Union;
        {
            center_piece,
            wing.clone(),
            scad!(Mirror(vec3(1.0, 0.0, 0.0));
                wing.clone()
            )
        }),
    })
}

#[cfg(test)]
mod tests {
    use scad_generator::*;

    use super::*;

    #[test]
    fn wing_nut_test() {
        let mut sfile = ScadFile::new();

        sfile.set_detail(50);

        sfile.add_object(wing_nut(5.8, 3.0));

        sfile.write_to_file(String::from("test_wing_nut.scad"));
    }
}
