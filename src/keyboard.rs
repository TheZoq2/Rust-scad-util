use scad::*;

pub const SWITCH_SPACING: f32 = 18.15;

pub fn mx_switch_hole() -> ScadObject 
{
    let inner_height = 1.5;
    let padding = 0.2;
    let inner_width = 14. + padding * 2.;


    let cube = scad!(Cube(vec3(inner_width, inner_width, inner_height)));

    scad!(Translate(vec3(-inner_width / 2., -inner_width / 2., 0.));
    {
        cube
    })
}
