pub mod areas_volumes;
pub use crate::areas_volumes::*;

pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
	let object_area = match objects {
		GeometricalShapes::Square => square_area(a),
		GeometricalShapes::Circle => circle_area(a) as usize,
		GeometricalShapes::Rectangle => rectangle_area(a, b),
		GeometricalShapes::Triangle => triangle_area(a, b) as usize,
	};
	if x * y >= times * object_area {
		true
	} else {
		false
	}
}

pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
	let shape_volume = match objects {
		GeometricalVolumes::Cube => cube_volume(a) ,
		GeometricalVolumes::Sphere => sphere_volume(a) as usize,
		GeometricalVolumes::Cone => cone_volume(a, b) as usize,
		GeometricalVolumes::Pyramid => triangular_pyramid_volume(a as f64, b) as usize,
		GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c),	
	};
	if x * y * z >= times * shape_volume {
		true
	} else {
		false
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
		println!(
			"Do 100 rectangles (2x1) fit in a 2 by 4 square? {}",
			area_fit(2, 4, GeometricalShapes::Rectangle, 100, 2, 1)
		);
		println!(
			"Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? {}",
			area_fit(5, 5, GeometricalShapes::Triangle, 3, 5, 3)
		);
		println!(
			"Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? {}",
			volume_fit(5, 5, 5, GeometricalVolumes::Sphere, 3, 2, 0, 0)
		);
		println!(
			"Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? {}",
			volume_fit(5, 7, 5, GeometricalVolumes::Parallelepiped, 1, 6, 7, 4)
		);
    }
}
