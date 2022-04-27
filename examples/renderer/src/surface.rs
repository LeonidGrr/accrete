use accrete::{Planetesimal, Ring};
use bevy::prelude::Color;

pub fn get_planet_color(planet: &Planetesimal) -> Color {
    if planet.is_gas_giant {
        return Color::RED;
    }
    if planet.is_moon {
        return Color::BLUE;
    }
    if planet.is_dwarf_planet {
        return Color::GRAY;
    }
    Color::WHITE
}

pub fn get_ring_color(ring: &Ring, planet: &Planetesimal) -> Color {
    Color::WHITE
}
