mod canvas;
mod color;
mod matrix;
mod tuple;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::tuple::Tuple;

use std::fs::File;
use std::io::Write;


struct Env {
    wind: Tuple,
    gravity: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn main() -> std::io::Result<()> {
    let mut proj = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 2.8, 0.0).normalize() * 11.25,
    };

    let env = Env {
        wind: Tuple::vector(0.0, -0.1, 0.0),
        gravity: Tuple::vector(-0.0, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    loop {
        proj.tick(&env);
        println!("{}, {}", proj.position.x as usize, (900_f32 - proj.position.y) as usize);
        canvas.write_pixel(proj.position.x as usize, (900_f32 - proj.position.y) as usize, Color::new(1.0, 0.0, 0.0));

        if proj.position.y <= 0.0 {
            break;
        }
    }

    println!("Hit the ground at {:?}", proj.position);

    let mut file = File::create("output.ppm")?;
    file.write_all(canvas.to_ppm().as_bytes())?;
    Ok(())
}

impl Projectile {
    fn tick(&mut self, env: &Env) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}
