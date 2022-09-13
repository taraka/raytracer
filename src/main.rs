mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod ray;
mod sphere;
mod tuple;
mod world;

type FP = f64;
const EPSILON: FP = 0.00001;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::*;

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z: FP = 10.0;
    let wall_size: FP = 10.0;
    let canvas_pixels: usize = 1000;
    let pixel_size = wall_size / (canvas_pixels as FP);
    let half: FP = (wall_size as FP) / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Sphere::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);

    let light_position = point(-10.0, 10.0, -10.0);
    let light_color = Color::white();
    let light = PointLight::new(light_position, light_color);

    // shape.set_transform(
    //     shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0),
    // );

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as FP);
        for x in 0..canvas_pixels {
            let world_x = (-half) + pixel_size * (x as FP);
            let position = point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&ray);

            if xs.len() != 0 {
                let point = ray.position(xs[0].t);
                let normal = xs[0].obj.normal_at(point);
                let eye = -ray.direction;
                let color = xs[0].obj.material.lighting(light, point, eye, normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut file = File::create("output.ppm")?;
    file.write_all(canvas.to_ppm().as_bytes())?;
    Ok(())
}
