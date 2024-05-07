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
    let rectangle_surface = x * y;
    let object_area = match objects {
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Circle => circle_area(a) as usize,
        GeometricalShapes::Rectangle => rectangle_area(a, b),
        GeometricalShapes::Triangle => triangle_area(a, b) as usize,
    };
    object_area * times <= rectangle_surface
}

pub struct FitParams {
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
}

pub fn volume_fit(params: FitParams) -> bool {
    let box_volume = params.x * params.y * params.z;
    let object_volume = match params.objects {
        GeometricalVolumes::Cube => cube_volume(params.a),
        GeometricalVolumes::Sphere => sphere_volume(params.a) as usize,
        GeometricalVolumes::Cone => cone_volume(params.a, params.b) as usize,
        GeometricalVolumes::Pyramid => triangular_pyramid_volume(params.a as f64, params.b) as usize,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(params.a, params.b, params.c),
    };
    object_volume * params.times <= box_volume
}
