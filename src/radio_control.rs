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





//qstruct!(Naze32()
//{
//    thickness: f32 = 2.,
//    width: f32 = 36.,
//});
//
//impl Naze32
//{
//    pub fn board() -> ScadObject
//    {
//        scad!(Union)
//    }
//
//    pub fn holes(height: f32) -> ScadObject
//    {
//        let distance = 30.;
//        let diameter = 3.;
//
//        let mut result = scad!(Union);
//
//        for x in vec!(-1., 1.)
//        {
//            for y in vec!(-1., 1.)
//            {
//                let hole = scad!(Cylinder(height, Diameter(diameter)));
//
//                result.add_child(scad!(Translate(vec3(x * distance / 2., y * distance / 2., 0.));{
//                    hole
//                }));
//            }
//        }
//        result
//    }
//}
