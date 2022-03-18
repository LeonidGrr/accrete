use accrete::{DustBand, PrimaryStar};
use macroquad::prelude::*;

pub trait Render {
    fn render(&self) {}
}

impl Render for DustBand {
    fn render(&self) {
        let DustBand {
            inner_edge,
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

pub fn get_scale_factor() -> f32 {
    // let outer_a = match system.planets.last() {
    //     // Some(p) => p.a as f32 * 3.3,
    //     None => 200.0,
    // };
    let outer_a = 150.0;
    screen_width() / outer_a
}
