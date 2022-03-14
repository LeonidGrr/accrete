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
    pub fn new(planet: &'a Planetesimal) -> Self {
        let Planetesimal { a, b, .. } = planet;
        let orbit = Orbit::new(*a, *b);
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
        let Planet { s, xi, vx, position, orbit, .. } = self;
        let Orbit { a, b, focus, positions } = orbit;

        let a = *a;
        let b = *b;
        let focus = *focus;
        let ba: f32 = a - 0.001;    // Boundary for a
        let u: f32 = 1.0;   // Gravitational parameter (M*6.67e-11)
        let t: f32 = 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5);    // Orbital period
        let dt: f32 = 10000.0 / t;  // A slice of time
        let mut px = *xi + *vx * dt;
        if *s * px > ba {
            px = *s * ba;
            *s = -*s;
        }
        let py = *s * b * ((1.0 - px.powf(2.0) / a / a) as f32).powf(0.5);
        *vx = py / b * (u * a / ((px - focus).powf(2.0) + py.powf(2.0))).powf(0.5);
        *xi = px;

        *position = (px, py);
    }

    pub fn render(&mut self, screen: &(f32, f32)) {
        self.get_position();
        let (x, y) = self.position;
        draw_circle(screen.0 + x - self.orbit.focus, screen.1 + y, 3.0, RED);
    }
}