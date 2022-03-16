use macroquad::prelude::*;

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
    // A slice of time
    pub dt: f32,
}

impl Orbit {
    pub fn new(a: f32, b: f32) -> Self {
        let focus = (a.powf(2.0) - b.powf(2.0)).sqrt();
        let ba: f32 = a - 0.001;
        let u: f32 = 1.0;
        let t: f32 = 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5);
        let dt: f32 = 10000.0 / t;
        let mut orbit = Orbit {
            a,
            b,
            focus,
            positions: vec![],
            ba,
            u,
            t,
            dt,
        };
        orbit.get_positions();
        orbit
    }

    pub fn get_positions(&mut self) {
        let Orbit { a, b, focus, ba, u, dt, .. } = *self;
        let mut s = 1.0;
        let mut xi = -(a - 0.001);
        let mut vx = 0.0;
        let mut next_positions = vec![];

        for key in 0..(a as usize * 10) {
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

    pub fn render(&self, screen: &(f32, f32)) {
        self.positions.windows(2).for_each(|v| {
            let (x1, y1) = v[0];
            let (x2, y2) = v[1];
            draw_line(screen.0 + x1 - self.focus, screen.1 + y1, screen.0 + x2 - self.focus, screen.1 + y2, 0.2, WHITE);
        });
    }
}