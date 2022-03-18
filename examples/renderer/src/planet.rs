use crate::orbit::Orbit;
use crate::render::get_scale_factor;
use accrete::Planetesimal;
use macroquad::prelude::*;

pub struct Planet<'a> {
    pub planet: &'a Planetesimal,
    pub position: (f32, f32),
    pub orbit: Orbit,
    pub s: f32,
    pub x_index: f32,
    pub vx: f32,
}

impl<'a> Planet<'a> {
    pub fn new(planet: &'a Planetesimal) -> Self {
        let Planetesimal { a, b, .. } = *planet;
        let scale_factor = get_scale_factor();
        let a = a as f32 * scale_factor;
        let b = b as f32 * scale_factor;
        let orbit = Orbit::new(a, b);
        let x_index = -(orbit.a - 0.001);
        let mut p = Planet {
            planet,
            orbit,
            position: (0.0, 0.0),
            s: 1.0,
            x_index,
            vx: 0.0,
        };
        p.get_position();
        p
    }

    pub fn get_position(&mut self) {
        let Planet { orbit, .. } = self;
        let Planet { s, x_index, vx, .. } = *self;
        let Orbit {
            a,
            b,
            focus,
            ba,
            u,
            dt,
            ..
        } = *orbit;

        let mut px = x_index + vx * dt;
        if s * px > ba {
            px = s * ba;
            self.s = -s;
        }
        let py = self.s * b * ((1.0 - px.powf(2.0) / a / a) as f32).powf(0.5);
        self.vx = py / b * (u * a / ((px - focus).powf(2.0) + py.powf(2.0))).powf(0.5);
        self.x_index = px;

        self.position = (px, py);
    }

    pub fn render(&mut self) {
        self.get_position();
        let (x, y) = self.position;
        draw_sphere(vec3(x - self.orbit.focus, y, 0.0), 1.0, None, BLUE);
    }
}
