use raytracer::tuple::Tuple;

struct Env {
    wind: Tuple,
    gravity: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn main() {
    let mut proj = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    let env = Env {
        wind: Tuple::vector(0.0, -0.1, 0.0),
        gravity: Tuple::vector(-0.0, 0.0, 0.0),
    };

    loop {
        proj.tick(&env);
        if proj.position.y <= 0.0 {
            break;
        }
    }

    println!("Hit the ground at {:?}", proj.position);
}

impl Projectile {
    fn tick(&mut self, env: &Env) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}
