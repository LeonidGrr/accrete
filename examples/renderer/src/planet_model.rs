use crate::{render::get_scale_factor, coalescence::Coalescence};
use accrete::Planetesimal;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct PlanetModel {
    pub planet_id: String,
    pub position: Vec3,
    pub s: f32,
    pub vx: f32,
    pub a: f32,
    pub b: f32,
    pub coalescence_target: Option<Vec3>,
}

impl PlanetModel {
    pub fn new(planet: &Planetesimal, dt: f32) -> Self {
        let Planetesimal { a, b, .. } = *planet;
        let scale_factor = get_scale_factor();
        let a = a as f32 * scale_factor;
        let b = b as f32 * scale_factor;
        let mut p = PlanetModel {
            planet_id: planet.id.clone(),
            position: vec3(-(a - 0.001), 0.0, 0.0),
            s: 1.0,
            vx: 0.0,
            a,
            b,
            coalescence_target: None,
        };
        p.update_position(dt);
        p
    }

    pub fn update_position(&mut self, dt: f32) {
        let PlanetModel { position, .. } = self;
        let PlanetModel { s, vx, a, b, .. } = *self;
        let focus = (a.powf(2.0) - b.powf(2.0)).sqrt();
        let ba: f32 = a - 0.001;
        let u: f32 = 1.0;
        let t: f32 = 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5);
        let dt: f32 = dt * 1000.0 / t;
        position.x = position.x + vx * dt;
        if s * position.x > ba {
            position.x = s * ba;
            self.s = -s;
        }
        position.y = self.s * b * ((1.0 - position.x.powf(2.0) / a.powf(2.0)) as f32).powf(0.5);
        self.vx = position.y / b * (u * a / ((position.x - focus).powf(2.0) + position.y.powf(2.0))).powf(0.5);
    }

    pub fn update_position_by_target(&mut self, target: Vec3) {
        let direction = (target - self.position).normalize();
        self.position = self.position + direction * 0.5;
    }

    pub fn render(&mut self) {
        draw_sphere(self.position, 1.0, None, BLUE);
    }
}
