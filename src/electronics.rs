use scad_generator::*;

pub fn teensy_lc() -> ScadObject
{
    let width = 18.0;
    let length = 36.0;
    let height = 2.0;

    let usb_width = 8.5;
    let usb_height = 4.;
    let usb_length = 6.;
    let usb_offset = 2.;
    let usb_bottom_offset = 0.5;


    let pcb = 
    {
        let cube = scad!(Cube(vec3(width, length, height)));

        scad!(Translate(vec3(-width / 2., 0., 0.)); cube)
    };

    let usb_port = 
    {
        let cube = scad!(Cube(vec3(usb_width, usb_length, usb_height)));
        
        let centered = scad!(Translate(vec3(-usb_width / 2., 0., 0.)); cube);
        scad!(Translate(vec3(0., -usb_offset, height - usb_bottom_offset)); centered)
    };

    scad!(Union;
    {
        pcb,
        usb_port
    })
}
