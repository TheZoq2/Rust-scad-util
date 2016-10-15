use scad_generator::*;

pub fn micro_usb_port() -> ScadObject
{
    let width = 8.5;
    let height = 4.;
    let length = 6.;

    let cube = scad!(Cube(vec3(width, length, height)));
    
    scad!(Translate(vec3(-width / 2., 0., 0.)); cube)
}


pub fn teensy_lc() -> ScadObject
{
    let width = 18.0;
    let length = 36.0;
    let height = 2.0;

    let usb_offset = 2.;
    let usb_bottom_offset = 0.5;


    let pcb = 
    {
        let cube = scad!(Cube(vec3(width, length, height)));

        scad!(Translate(vec3(-width / 2., 0., 0.)); cube)
    };

    let usb_port = scad!(Translate(vec3(0., -usb_offset, height - usb_bottom_offset)); micro_usb_port());

    scad!(Union;
    {
        pcb,
        usb_port
    })
}



pub struct PolouMicroUsbBreakout
{
    pub width: f32,
    pub length: f32,
    pub height: f32,

    pub hole_diameter: f32,
    pub hole_separation: f32,
    pub hole_distance_from_back: f32,
}

impl PolouMicroUsbBreakout
{
    pub fn new() -> PolouMicroUsbBreakout
    {
        PolouMicroUsbBreakout {
            width: 15.,
            length: 9.,
            height: 1.,

            hole_diameter: 2.,
            hole_separation: 12.,
            hole_distance_from_back: 2.,
        }
    }

    //Remember USB port offset
    pub fn board(&self) -> ScadObject
    {
        let usb_offset = 1.5;
        let usb_bottom_offset = 0.5;

        let cutout_holes = self.hole_shape(self.height);

        let pcb = 
        {
            let cube = scad!(Cube(vec3(self.width, self.length, self.height)));

            //center it
            let centered = scad!(Translate(vec3(-self.width / 2., 0., 0.)); cube);

            scad!(Difference;
            {
                centered,
                cutout_holes
            })
        };

        let usb_port = scad!(Translate(vec3(0., -usb_offset, self.height - usb_bottom_offset)); micro_usb_port());

        scad!(Union;
        {
            pcb,
            usb_port
        })
    }

    pub fn hole_shape(&self, height: f32) -> ScadObject
    {
        let cylinder = scad!(Cylinder(height, Diameter(self.hole_diameter)));

        let cube = scad!(Translate(vec3(0., -self.hole_diameter / 2., 0.));
        {
            scad!(Cube(vec3((self.width - self.hole_separation) / 2., self.hole_diameter, self.height)))
        });

        let cutout_back_offset = self.hole_distance_from_back + self.hole_diameter / 2.;
        let cutout = scad!(Translate(vec3(self.hole_separation / 2., cutout_back_offset, 0.));
        {
            cylinder, 
            cube
        });

        scad!(Union;
        {
            scad!(Mirror(vec3(1., 0., 0.));
            {
                cutout.clone()
            }),
            cutout
        })
    }
}




#[cfg(test)]
mod tests
{
    use super::*;
    use scad_generator::*;


    #[test]
    fn polou_breakout_test()
    {
        {
            let mut sfile = ScadFile::new();

            sfile.set_detail(10);
            sfile.add_object(PolouMicroUsbBreakout::new().board());
            sfile.write_to_file(String::from("usb_breakout.scad"));
        };

        {
            let mut sfile = ScadFile::new();

            sfile.set_detail(10);
            sfile.add_object(PolouMicroUsbBreakout::new().wall_mount());
            sfile.write_to_file(String::from("usb_breakout_mount.scad"));
        };
    }
}
