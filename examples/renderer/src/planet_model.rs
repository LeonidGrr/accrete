use crate::render::get_scale_factor;
use accrete::Planetesimal;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct PlanetModel {
    pub planet_id: String,
    pub position: Vec3,
    pub s: f32,
    pub vx: f32,
    pub a: f32,
    pub e: f32,
    pub t: f32,
    pub u: f32,
    pub coalescence_a: Option<f32>,
}

impl PlanetModel {
    pub fn new(planet: &Planetesimal, dt: f32) -> Self {
        let Planetesimal { a, e, .. } = *planet;
        let scale_factor = get_scale_factor();
        let a = a as f32 * scale_factor;
        let u: f32 = 1.0;
        let mut p = PlanetModel {
            planet_id: planet.id.clone(),
            position: vec3(-(a - 0.001), 0.0, 0.0),
            s: 1.0,
            vx: 0.0,
            a,
            u,
            e: e as f32,
            coalescence_a: None,
            t: 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5),
        };
        p.update_position(dt);
        p
    }

    pub fn update_position(&mut self, dt: f32) {
        let PlanetModel { position, .. } = self;
        let PlanetModel { s, vx, a, e, t, u, .. } = *self;
        let b = a * (1.0 - e.powf(2.0)).sqrt();
        let focus = (a.powf(2.0) - b.powf(2.0)).sqrt();
        let ba: f32 = a - 0.001;
        let dt: f32 = dt * 10000.0 / t;
        position.x = position.x + vx * dt;
        if s * position.x > ba {
            position.x = s * ba;
            self.s = -s;
        }
        position.y = self.s * b * ((1.0 - position.x.powf(2.0) / a.powf(2.0)) as f32).powf(0.5);
        self.vx = position.y / b * (u * a / ((position.x - focus).powf(2.0) + position.y.powf(2.0))).powf(0.5);
    }

    pub fn update_a(&mut self, target_a: f32) {
        debug!("{} - {}", self.a, target_a);
        debug!("{}", self.position);
        let coalesce_distance = (target_a - self.a).abs();
        if coalesce_distance > 0.5 {
            match self.a < target_a {
                true => self.a += 0.1,
                false => self.a -= 0.1,
            }
        }
    }

    pub fn render(&mut self) {
        draw_sphere(self.position, 1.0, None, BLUE);
    }
}
