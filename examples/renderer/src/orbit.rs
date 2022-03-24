use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Orbit {
    pub a: f32,
    pub b: f32,
    pub focus: f32,
    pub positions: Vec<(f32, f32)>,
    // Boundary for a
    pub ba: f32,
    // Gravitational parameter (M*6.67e-11)
    pub u: f32,
    // Orbital period
    pub t: f32,
}

impl Orbit {
    pub fn _new(a: f32, b: f32, dt: f32) -> Self {
        let focus = (a.powf(2.0) - b.powf(2.0)).sqrt();
        let ba: f32 = a - 0.001;
        let u: f32 = 1.0;
        let t: f32 = 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5);
        let dt: f32 = dt * 1000.0 / t;
        let mut orbit = Orbit {
            a,
            b,
            focus,
            positions: vec![],
            ba,
            u,
            t,
        };
        orbit._update_positions(dt);
        orbit
    }

    pub fn _update_positions(&mut self, dt: f32) {
        let Orbit {
            a,
            b,
            focus,
            ba,
            u,
            ..
        } = *self;
        let mut s = 1.0;
        let mut xi = -(a - 0.001);
        let mut vx = 0.0;
        let mut next_positions = vec![];
        for _ in 0..(a as usize * 10) {
            let mut px = xi + vx * dt * 100.0;
            if s * px > ba {
                px = s * ba;
                s = -s;
            }
            let py = s * b * ((1.0 - px.powf(2.0) / a / a) as f32).powf(0.5);
            vx = py / b * (u * a / ((px - focus).powf(2.0) + py.powf(2.0))).powf(0.5);
            xi = px;
            next_positions.push((px, py));
        }

        self.positions = next_positions;
    }
}
