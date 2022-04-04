use crate::{orbit::Orbit, planet_model::PlanetModel, state::State};
use accrete::{DustBand, PrimaryStar};
use macroquad::prelude::*;

pub trait Render {
    fn render(&self) {}
}

impl Render for DustBand {
    fn render(&self) {
        let DustBand {
            outer_edge,
            dust_present,
            ..
        } = *self;
        let scale_factor = get_scale_factor();
        let color = match dust_present {
            true => Color::new(0.5, 0.5, 0.5, 0.2),
            false => DARKGRAY,
        };
        draw_poly(0.0, 0.0, 100, outer_edge as f32 * scale_factor, 0.0, color);
    }
}

impl Render for PrimaryStar {
    fn render(&self) {
        let [r, g, b] = self.color;
        let color = Color::new(r as f32, g as f32, b as f32, 1.0);
        draw_sphere(vec3(0., 0., 0.), 1.0, None, color);
    }
}

impl Render for Orbit {
    fn render(&self) {
        let color = Color::new(1.0, 1.0, 1.0, 0.25);
        self.positions.windows(2).for_each(|v| {
            let (x1, y1) = v[0];
            let (x2, y2) = v[1];
            draw_line(x1 - self.focus, y1, x2 - self.focus, y2, 0.2, color);
        });
    }
}

pub fn get_scale_factor() -> f32 {
    // let outer_a = match system.planets.last() {
    //     Some(p) => p.a as f32,
    //     None => 50.0,
    // };
    // screen_width() / outer_a
    screen_width() / 75.0
}

impl Render for State {
    fn render(&self) {
        for p in self.planet_models.values() {
            p.render();
            // p.orbit.render();
        }
    }
}

impl Render for PlanetModel {
    fn render(&self) {
        let mut color = BLUE;

        if self.planet.has_collision {
            color = RED;
        }

        if self.planet.is_gas_giant {
            color = GREEN;
        };

        if self.planet.is_moon {
            color = PINK;
        };

        if self.planet.is_dwarf_planet {
            return;
        }

        draw_sphere(self.position, 1.0, None, color);

        for m in self.moon_models.values() {
            m.render();
        }
    }
}
