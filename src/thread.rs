use scad_generator::*;

pub fn thread(height: f32, radius: f32, thread_width: f32) -> ScadObject
{
    let square = scad!(Square(vec2(thread_width, thread_width)));
    
    scad!(LinearExtrude(LinExtrudeParams{twist: 1080, .. Default::default}){
        square,
    })
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

        sfile.add_object(thread(20., 5., 1.));

        sfile.write_to_file("thread_test.scad");
    }
}
