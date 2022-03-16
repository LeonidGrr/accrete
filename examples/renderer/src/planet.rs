use accrete::Planetesimal;
use crate::orbit::Orbit;
use macroquad::prelude::*;

pub struct Planet<'a> {
    pub planet: &'a Planetesimal,
    pub position: (f32, f32),
    pub orbit: Orbit,
    pub s: f32,
    // x index
    pub xi: f32,
    // x component of orbital velocity
    pub vx: f32,
}

impl<'a> Planet<'a> {
    pub fn new(planet: &'a Planetesimal, scale_factor: f32) -> Self {
        let Planetesimal { a, b, .. } = *planet;
        let a = a as f32 * scale_factor;
        let b = b as f32 * scale_factor;
        let orbit = Orbit::new(a, b);
        let xi = -(orbit.a - 0.001);
        let mut p = Planet {
            planet,
            orbit,
            position: (0.0, 0.0),
            s: 1.0,
            xi,
            vx: 0.0,
        };
        p.get_position();
        p
    }

    pub fn get_position(&mut self) {
        let Planet { orbit, .. } = self;
        let Planet { s, xi, vx, .. } = *self;
        let Orbit { a, b, focus, ba, u, dt, .. } = *orbit;

        let mut px = xi + vx * dt;
        if s * px > ba {
            px = s * ba;
            self.s = -s;
        }
        let py = self.s * b * ((1.0 - px.powf(2.0) / a / a) as f32).powf(0.5);
        self.vx = py / b * (u * a / ((px - focus).powf(2.0) + py.powf(2.0))).powf(0.5);
        self.xi = px;

        self.position = (px, py);
    }

    pub fn render(&mut self, screen: &(f32, f32)) {
        self.get_position();
        let (x, y) = self.position;
        draw_circle(screen.0 + x - self.orbit.focus, screen.1 + y, 3.0, RED);
    }
}