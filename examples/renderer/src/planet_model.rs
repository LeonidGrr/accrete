use std::collections::HashMap;

use crate::{render::get_scale_factor, state::PlanetModels};
use accrete::Planetesimal;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct PlanetModel {
    pub planet: Planetesimal,
    pub moon_models: PlanetModels,
    pub position: Vec3,
    pub barycenter: Vec3,
    pub a: f32,
    pub e: f32,
    pub target_a: Option<f32>,
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
        let moon_models = planet.moons.iter().fold(HashMap::new(), |mut acc, m| {
            let moon = PlanetModel::new(m.clone(), time);
            acc.insert(moon.id.clone(), moon);
            acc
        });
        let mut p = PlanetModel {
            planet,
            moon_models,
            position: vec3(-(a - 0.001), 0.0, 0.0),
            barycenter: vec3(0.0, 0.0, 0.0),
            a,
            e: e as f32,
            target_a: None,
        };

        p.update_position(time);
        p
    }

    pub fn update_position(&mut self, time: f64) {
        let PlanetModel {
            position,
            barycenter,
            moon_models,
            ..
        } = self;
        let PlanetModel { a, e, .. } = *self;
        let b = a * (1.0 - e.powf(2.0)).sqrt();
        let u: f32 = 1.0;
        let focus = (a.powf(2.0) - b.powf(2.0)).sqrt();
        let t = 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5);
        let current_t = (time / (t * 360.0) as f64) * 100000.0;

        position.x = barycenter.x + focus + (a as f64 * current_t.cos()) as f32;
        position.y = barycenter.y + (b as f64 * current_t.sin()) as f32;

        for m in moon_models.values_mut() {
            m.barycenter = *position;
            m.update_position(time);
        }
        // TODO speed up near star
    }

    pub fn update_a(&mut self) {
        if let Some(target_a) = self.target_a {
            let coalesce_distance = (target_a - self.a).abs();
            if coalesce_distance > 1.0 {
                match self.a < target_a {
                    true => {
                        self.a += 0.5;
                    }
                    false => {
                        self.a -= 0.5;
                    }
                }
            }
        }
    }
}
