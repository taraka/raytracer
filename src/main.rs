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
use crate::pattern::*;
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

    let mut p1 = Pattern::stripe(Color::white(), Color::new(0.0, 0.3, 0.6));
    p1.transform = rotation_y(PI / 2.0);

    let floor_pattern = Pattern::blended(
        Pattern::stripe(Color::white(), Color::new(0.0, 0.2, 0.6)),
        p1,
    );

    let mut floor = Shape::plane();
    // floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material.pattern = floor_pattern;
    floor.material.specular = 0.0;

    world.objects.push(floor.clone());

    let mut middle = Shape::sphere();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.pattern = Pattern::radialgradient(Color::white(), Color::new(0.5, 0.0, 0.5));
    middle.material.pattern.transform = scaling(0.1, 0.1, 0.1) * rotation_x(PI / 2.0);
    middle.material.diffuse = 0.6;
    middle.material.specular = 0.7;

    world.objects.push(middle);

    let mut right = Shape::sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material.pattern = Pattern::checkers(Color::new(0.1, 1.0, 0.5), Color::new(1.0, 0.0, 0.5));
    right.material.pattern.transform = scaling(0.2, 0.2, 0.2);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    world.objects.push(right);

    let mut left = Shape::sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.pattern = Pattern::ring(Color::new(1.0, 0.8, 0.1), Color::white());
    left.material.pattern.transform = scaling(0.1, 0.1, 0.1) * rotation_x(PI / 2.0);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    world.objects.push(left);

    world.light = Some(PointLight::new(
        point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(1000, 600, PI / 3.0);
    camera.transform = Matrix4::view_transform(
        point(0.0, 1.7, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let mut file = File::create("output.ppm")?;
    file.write_all(canvas.to_ppm().as_bytes())?;
    Ok(())
}
