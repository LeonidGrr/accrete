use std::collections::HashMap;
use macroquad::prelude::*;

pub struct Orbit {
    pub a: f32,
    pub b: f32,
    pub focus: f32,
    pub positions: HashMap<usize, (f32, f32)>,
}

impl Orbit {
    pub fn new(a: f64, b: f64) -> Self {
        let a = a as f32 * 4.0;
        let b = b as f32 * 4.0;
        let focus = (a.powf(2.0) - b.powf(2.0)).sqrt();
        let positions = Orbit::get_positions(a, b, focus);
        let mut orbit = Orbit {
            a,
            b,
            focus,
            positions,
        };
        orbit
    }

    pub fn get_positions(a: f32, b: f32, focus: f32) -> HashMap<usize, (f32, f32)> {
        let mut s = 1.0;
        let mut xi = -(a - 0.001);
        let mut vx = 0.0;
        let ba: f32 = a - 0.001;    // Boundary for a
        let u: f32 = 1.0;   // Gravitational parameter (M*6.67e-11)
        let t: f32 = 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5);    // Orbital period
        let dt: f32 = 10000.0 / t;  // A slice of time
        let mut positions = HashMap::new();

        for key in 0..(a as usize * 100) {
            let mut px = xi + vx * dt;
            if s * px > ba {
                px = s * ba;
                s = -s;
            }
            let py = s * b * ((1.0 - px.powf(2.0) / a / a) as f32).powf(0.5);
            vx = py / b * (u * a / ((px - focus).powf(2.0) + py.powf(2.0))).powf(0.5);
            xi = px;
            positions.insert(key, (px, py));
        }

        positions
    }

    pub fn render(&self, screen: &(f32, f32)) {
        for (_, (x, y)) in self.positions.iter() {
            draw_circle(screen.0 + x - self.focus, screen.1 + y, 0.2, WHITE);
        }
    }
}