extern crate nalgebra as na;

use scad::*;

use std::vec::Vec;

use std::f32::consts;

pub fn thread(height: f32, radius: f32, thread_width: f32, thread_height: f32) -> ScadObject
{
    let segment_amount = 30;

    //The steps that will be taken for each segment
    let height_step = thread_height / segment_amount as f32;
    let angle_step = consts::PI * 2. / segment_amount as f32;

    //The amount of rings that has to be created to reach the desired height
    let ring_amount = (height/thread_height).trunc() as i32 + 2;

    let mut points = Vec::<na::Vector3<f32>>::new();
    
    let inner_radius = radius;
    let outer_radius = radius + thread_width;
    for i in 0 .. (ring_amount * segment_amount)
    {
        //The tiny number here is needed because scad interprets the shape as intersecting itself
        //otherwise
        let curr_height = (height_step + 0.0001) * i as f32;
        let curr_angle = angle_step * i as f32;
        
        points.push(vec3(curr_angle.cos() * inner_radius, curr_angle.sin() * inner_radius, curr_height));
        points.push(vec3(curr_angle.cos() * outer_radius, curr_angle.sin() * outer_radius, curr_height + thread_height / 2.));
        points.push(vec3(curr_angle.cos() * inner_radius, curr_angle.sin() * inner_radius, curr_height + thread_height));
    }

    let mut faces = Vec::<Vec<i32>>::new();
    
    //add the end caps
    faces.push(vec!(0, 2, 1));
    faces.push(vec!(points.len() as i32 - 1, points.len() as i32 - 3, points.len() as i32 - 2));

    let points_per_segment = 3;
    for i in 0 .. (ring_amount * segment_amount) - 1
    {
        let offset = i * points_per_segment;
        //bottom face
        faces.push(vec!(
                   (offset + 0),
                   (offset + 1),
                   (offset + 4),
                   (offset + 3),
                ));

        //Top face
        faces.push(vec!(
                   (offset + 1),
                   (offset + 2),
                   (offset + 5),
                   (offset + 4),
                ));
        //Back face
        faces.push(vec!(
                   (offset + 0),
                   (offset + 3),
                   (offset + 5),
                   (offset + 2),
                ));
    }
    let poly = scad!(Polyhedron(points, faces));

    //Cutting of the exess threads
    let translated = scad!(Translate(vec3(0., 0., -thread_height));{poly});
    let cutter = scad!(Cylinder(height, Radius(radius + thread_width * 2.)));

    //translated
    scad!(Intersection;{translated, cutter})
}


#[cfg(test)]
mod thread_test
{
    use scad::*;
    
    use super::*;

    #[test]
    fn thread_test()
    {
        let mut sfile = ScadFile::new();
        sfile.set_detail(30);

        let height = 10.0;
        let radius = 10.5;
        let thread = thread(height, radius, 1.5, 3.);

        //sfile.add_object(
        //    scad!(Union;{
        //        thread,
        //        scad!(Cylinder(height, Radius(radius + 0.01)))
        //    }));
        sfile.add_object(
            scad!(Difference;{
                scad!(Cylinder(height, Radius(radius + 5.))),
                thread,
                scad!(Cylinder(height, Radius(radius + 0.01)))
            }));

        sfile.write_to_file("thread_test.scad".to_string());
    }
}
