mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod pattern;
mod ray;
mod shape;
mod tuple;
mod world;

type FP = f64;
const EPSILON: FP = 0.00001;

use crate::camera::Camera;
use crate::color::Color;
use crate::light::PointLight;
use crate::matrix::*;
use crate::shape::Shape;
use crate::tuple::*;
use crate::world::World;
use std::f64::consts::PI;

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut world = World::new();

    let mut floor = Shape::plane();
    // floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    world.objects.push(floor.clone());

    // let mut left_wall = Sphere::new();
    // left_wall.transform = translation(0.0, 0.0, 5.0)
    //     * rotation_y(-PI / 4.0)
    //     * rotation_x(PI / 2.0)
    //     * scaling(10.0, 0.01, 10.0);
    // left_wall.material = floor.material;

    // world.objects.push(left_wall);

    // let mut right_wall = Sphere::new();
    // right_wall.transform = translation(0.0, 0.0, 5.0)
    //     * rotation_y(PI / 4.0)
    //     * rotation_x(PI / 2.0)
    //     * scaling(10.0, 0.01, 10.0);
    // right_wall.material = floor.material;

    // world.objects.push(right_wall);

    let mut middle = Shape::sphere();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.6;
    middle.material.specular = 0.7;

    world.objects.push(middle);

    let mut right = Shape::sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    world.objects.push(right);

    let mut left = Shape::sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    world.objects.push(left);

    world.light = Some(PointLight::new(
        point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(2000, 1000, PI / 3.0);
    camera.transform = Matrix4::view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let mut file = File::create("output.ppm")?;
    file.write_all(canvas.to_ppm().as_bytes())?;
    Ok(())
}
