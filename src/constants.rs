use na::Vector3;

// These are functions becasue const fucntions aren't allowed and vectors don't
// seem to have any constructors
pub fn x_axis() -> Vector3<f32> {Vector3::new(1., 0., 0.)}
pub fn y_axis() -> Vector3<f32> {Vector3::new(0., 1., 0.)}
pub fn z_axis() -> Vector3<f32> {Vector3::new(0., 0., 1.)}
