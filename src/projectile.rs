use crate::canvas::canvas_to_ppm;
use crate::canvas::Canvas;
use crate::colors::Color;
use crate::tuples::Coord;

#[derive(Debug)]
struct Projectile {
    position: Coord,
    velocity: Coord,
}

struct Environment {
    gravity: Coord,
    wind: Coord,
}

fn tick(env: &Environment, projectile: Projectile) -> Projectile {
    Projectile {
        position: &projectile.position + &projectile.velocity,
        velocity: &(&projectile.velocity + &env.gravity) + &env.wind,
    }
}

pub fn ch1() {
    let mut p = Projectile {
        position: Coord::point(0.0, 1.0, 0.0),
        velocity: Coord::vector(1.0, 1.0, 0.0).normalize(),
    };
    let e = Environment {
        gravity: Coord::vector(0.0, -0.1, 0.0),
        wind: Coord::vector(-0.01, 0.0, 0.0),
    };
    while p.position.y > 0.0 {
        println!("Current: {:?}", p);
        p = tick(&e, p);
    }
}

pub fn ch2() -> String {
    let mut p = Projectile {
        position: Coord::point(0.0, 1.0, 0.0),
        velocity: &Coord::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Coord::vector(0.0, -0.1, 0.0),
        wind: Coord::vector(0.01, 0.0, 0.0),
    };
    let mut c = Canvas::new(900, 550);
    while p.position.y > 0.0 {
        let red = Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        };
        c.write_pixel(
            p.position.x.round() as usize,
            c.height - (p.position.y.round() as usize),
            red,
        );
        p = tick(&e, p);
    }
    canvas_to_ppm(&c)
}
