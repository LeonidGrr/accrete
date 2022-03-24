use crate::render::get_scale_factor;
use accrete::Planetesimal;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct PlanetModel {
    pub planet: Planetesimal,
    pub position: Vec3,
    pub a: f32,
    pub e: f32,
    pub coalescence_a: Option<f32>,
}

impl std::ops::Deref for PlanetModel {
    type Target = Planetesimal;
    fn deref(&self) -> &Self::Target {
        &self.planet
    }
}

impl PlanetModel {
    pub fn new(planet: Planetesimal, time: f64) -> Self {
        let Planetesimal { a, e, .. } = planet;
        let scale_factor = get_scale_factor();
        let a = a as f32 * scale_factor;
        let mut p = PlanetModel {
            planet,
            position: vec3(-(a - 0.001), 0.0, 0.0),
            a,
            e: e as f32,
            coalescence_a: None,
        };

        p.update_position(time);
        p
    }

    pub fn update_position(&mut self, time: f64) {
        let PlanetModel { position, .. } = self;
        let PlanetModel { a, e, .. } = *self;
        let b = a * (1.0 - e.powf(2.0)).sqrt();
        let u: f32 = 1.0;
        let focus = (a.powf(2.0) - b.powf(2.0)).sqrt();
        let t = 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5);
        let current_t = (time / (t * 360.0) as f64) * 100000.0;
        position.x = focus + (a as f64 * current_t.cos()) as f32;
        position.y = (b as f64 * current_t.sin()) as f32;
    }

    pub fn update_a(&mut self, target_a: f32) {
        let coalesce_distance = (target_a - self.a).abs();
        if coalesce_distance > 1.0 {
            match self.a < target_a {
                true => {
                    self.a += 0.5;
                },
                false => {
                    self.a -= 0.5;
                },
            }
        }
    }

    pub fn render(&mut self) {
        let color = match self.planet.has_collision {
            true => RED,
            false => BLUE,
        };
        draw_sphere(self.position, 1.0, None, color);
    }
}
