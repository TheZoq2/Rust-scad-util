use scad_generator::*;

use std::f32::consts;

pub fn thread(height: f32, radius: f32, thread_width: f32) -> ScadObject
{
    let segment_amount = 20;
    let thread_height_factor = 0.5;
    let thread_height = thread_width * thread_height_factor;

    let circumference = radius * consts::PI * 2.;

    //The steps that will be taken for each segment
    let height_step = thread_height / segment_amount as f32;
    let angle_step = 360. / segment_amount as f32;

    //The length of a single segment in the thread
    let segment_length = (circumference) / segment_amount as f32;
    
    //The amount of rings that has to be created to reach the desired height
    let ring_amount = (height/thread_width).trunc() as i32 + 2;

    //The basic segment that makes up the threads
    let base_segment = {
        //Length added because we are calculating the length based on the center of the cube, not
        //the outside which leaves holes between the segments * by 2. again because we have to add
        //extra length to both sides
        let extra_length = ((thread_width * consts::SQRT_2) / 2.) * (angle_step.to_radians()).tan();

        let total_length = segment_length + extra_length;

        let cube = scad!(Cube(vec3(total_length, thread_width, thread_width)));
        let cube = scad!(Translate(vec3(-total_length/2., 0., 0.));{cube});

        let rotated = scad!(Rotate(45., vec3(1., 0., 0.));{cube});

        let squished = scad!(Scale(vec3(1., 1., thread_height_factor));{rotated});

        //Cutting off the extra part added to the end of each segment
        let cutter = {
            let cube = scad!(Cube(vec3(segment_length, radius * 2., thread_height * consts::SQRT_2)));

            let rotated = scad!(Rotate(-angle_step/2., vec3(0., 0., 1.));{cube});

            let mirrored = scad!(Mirror(vec3(1., 0., 0.));{rotated.clone()});

            scad!(Union;{rotated, mirrored})
        };
        let cut = scad!(Difference;{
            squished, 
            scad!(Translate(vec3(0., -radius, 0.));{cutter}),
        });

        let segment_angle = (thread_height).atan2(circumference).to_degrees();

        scad!(Rotate(segment_angle, vec3(0., 1., 0.));{cut})
    };

    let mut result = scad!(Union);

    for i in 0..(segment_amount * ring_amount)
    {
        //let translated_segment = result.add_child()
        let translated_segment = scad!(Translate(vec3(0., radius, height_step * i as f32));{base_segment.clone()});
        
        let rotated = scad!(Rotate(i as f32 * angle_step, vec3(0., 0., 1.));{translated_segment});

        result.add_child(rotated);
    }

    result = scad!(Translate(vec3(0., 0., -thread_height * consts::SQRT_2/ 2.));{result});

    let cutter = scad!(Cylinder(height, Radius(radius + thread_width)));
    result = scad!(Intersection;{result,cutter});

    result
}


#[cfg(test)]
mod thread_test
{
    use scad_generator::*;
    
    use super::*;

    #[test]
    fn thread_test()
    {
        let mut sfile = ScadFile::new();
        sfile.set_detail(30);

        sfile.add_object(thread(3.0, 5., 1.5));

        sfile.write_to_file("thread_test.scad".to_string());
    }
}
