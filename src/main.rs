mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod ray;
mod sphere;
mod tuple;

type FP = f64;
const EPSILON: FP = 0.00001;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z: FP = 10.0;
    let wall_size: FP = 10.0;
    let canvas_pixels: usize = 1000;
    let pixel_size = wall_size / (canvas_pixels as FP);
    let half: FP = (wall_size as FP) / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::red();
    let mut shape = Sphere::new();

    shape.set_transform(
        Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix4::scaling(0.5, 1.0, 1.0),
    );

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as FP);
        for x in 0..canvas_pixels {
            let world_x = (-half) + pixel_size * (x as FP);
            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(ray);

            if xs.len() != 0 {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut file = File::create("output.ppm")?;
    file.write_all(canvas.to_ppm().as_bytes())?;
    Ok(())
}
