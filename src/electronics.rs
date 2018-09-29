use scad::*;

pub fn micro_usb_port(extra_length: f32) -> ScadObject
{
    let width = 8.5;
    let height = 4.;
    let length = 6. + extra_length;

    let cube = scad!(Cube(vec3(width, length, height)));
    
    scad!(Translate(vec3(-width / 2., -extra_length, 0.)); cube)
}


pub fn teensy_lc(usb_extra_length: f32) -> ScadObject
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

    let usb_port = scad!(Translate(vec3(0., -usb_offset, height - usb_bottom_offset));
    {
        micro_usb_port(usb_extra_length)
    });

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
            hole_distance_from_back: 3.,
        }
    }

    pub fn board(&self, usb_extra_length: f32) -> ScadObject
    {
        let usb_offset = 1.5;
        let usb_bottom_offset = 0.5;

        let cutout_holes = self.hole_shape();

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

        let usb_port = scad!(Translate(vec3(0., -usb_offset, self.height - usb_bottom_offset));
        {
            micro_usb_port(usb_extra_length)
        });

        scad!(Union;
        {
            pcb,
            usb_port
        })
    }

    pub fn hole_shape(&self) -> ScadObject
    {
        let cylinder = scad!(Cylinder(self.height, Diameter(self.hole_diameter)));

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
    use scad::*;

    #[test]
    fn teensy_test()
    {
        {
            let mut sfile = ScadFile::new();

            sfile.set_detail(10);
            sfile.add_object(teensy_lc(5.));
            sfile.write_to_file(String::from("teensy.scad"));
        };
    }

    #[test]
    fn polou_breakout_test()
    {
        {
            let mut sfile = ScadFile::new();

            sfile.set_detail(10);
            sfile.add_object(PolouMicroUsbBreakout::new().board(10.));
            sfile.write_to_file(String::from("usb_breakout.scad"));
        };
    }
}
