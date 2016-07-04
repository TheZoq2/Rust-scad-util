use scad_generator::*;

pub fn generic_motor_holes(small_diameter: f32, big_diameter: f32, screw_diameter: f32) -> ScadObject 
{
    let center_hole_diameter = 9.0;

    let height = 30.0;
    let mut result = scad!(Translate(vec3(0.0,0.0,-height/2.0)));

    //Add the center hole
    result.add_child(scad!(Cylinder(height, Diameter(center_hole_diameter))));

    //Add the screwholes
    for i in &[-1.0,1.0]
    {
        result.add_child(
            scad!(Translate(vec3(i * small_diameter / 2.0, 0.0, 0.0));
            {
                scad!(Cylinder(height, Diameter(screw_diameter)))
            }),
        );
        result.add_child(
            scad!(Translate(vec3(0.0, i * big_diameter / 2.0, 0.0));
            {
                scad!(Cylinder(height, Diameter(screw_diameter)))
            }),
        );
    }

    return result;
}
