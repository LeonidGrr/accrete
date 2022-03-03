use accrete::events::{AccreteEvent, EVENTS};
use accrete::{Accrete, Planetesimal};
use macroquad::prelude::*;

struct Orbit {
    a: f32,
    b: f32,
    focus: f32
}

struct Planet<'a> {
    planet: &'a Planetesimal,
    position: [f32; 2],
    orbit: Orbit,
    s: f32,
    // x index
    xi: f32,
    // x component of orbital velocity
    vx: f32,
}

impl<'a> Planet<'a> {
    pub fn new(planet: &'a Planetesimal) -> Self {
        let Planetesimal { a, b, .. } = planet;
        let a = *a as f32 * 4.0;
        let b = *b as f32 * 4.0;
        let mut orbit = Orbit {
            a,
            b,
            focus: (a.powf(2.0) - b.powf(2.0)).sqrt(),
        };

        let mut p = Planet {
            planet,
            orbit,
            position: [0.0, 0.0],
            s: 1.0,
            xi: -(a - 0.001),
            vx: 0.0,
        };
        p.get_position();
        p
    }

    pub fn get_position(&mut self) {
        let Planet { s, xi, vx, planet, position, orbit } = self;
        let Orbit { a, b, focus } = orbit;
        let a = *a;
        let b = *b;
        let focus = *focus;
        let ba: f32 = a - 0.001;	// Boundary for a
        let u: f32 = 1.0;			// Gravitional parameter (M*6.67e-11)
        let t: f32 = 2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5);		// Orbital period
        let dt: f32 = 10000.0 / t;		// A slice of time		
        let mut px = *xi + *vx * dt;
        if *s * px > ba {
            px = *s * ba;
            *s = -*s;
        }
        let py = *s * b * ((1.0 - px.powf(2.0) / a / a) as f32).powf(0.5);
        *vx = py / b * (u * a / ((px - focus).powf(2.0) + py.powf(2.0))).powf(0.5);
        *xi = px;

        *position = [px, py];
    }
}

struct State<'a> {
    planets: Vec<Planet<'a>>,
    event_idx: usize,
    current_event: &'a AccreteEvent,
    step: f64,
}

impl State<'_> {
    fn event_handler(&mut self) {
        match self.current_event {
            // AccreteEvent::PlanetarySystemSetup(name, _) => name,
            AccreteEvent::PlanetesimalCreated(name, planet) => self.planets.push(Planet::new(planet)),
            // AccreteEvent::PlanetesimalAccretedDust(name, _) => name,
            // AccreteEvent::PlanetesimalToGasGiant(name, _) => name,
            // AccreteEvent::DustBandsUpdated(name, _) => name,
            // AccreteEvent::PlanetesimalsCoalesced(name, _, _, _) => name,
            // AccreteEvent::PlanetesimalCaptureMoon(name, _, _, _) => name,
            // AccreteEvent::PlanetesimalMoonToRing(name, _) => name,
            // AccreteEvent::PostAccretionStarted(name) => name,
            // AccreteEvent::OuterBodyInjected(name, _) => name,
            // AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            // AccreteEvent::PlanetarySystemComplete(name, _) => name,
            _ => ()
        }
}
}
#[macroquad::main("Accrete")]
async fn main() {
    let mut accrete = Accrete::new(33);
    accrete.stellar_mass = 2.0;
    accrete.planetary_system();

    let log = EVENTS.lock().unwrap();
    println!("Total {:#?} events.", log.len());

    for event in log.iter() {
        match event {
            AccreteEvent::PlanetarySystemSetup(s1, _) => println!("{:#?}", s1),
            AccreteEvent::PlanetarySystemComplete(s1, _) => println!("{:#?}", s1),
            _ => (),
        }
    }

    let mut state = State {
        step: 1.0,
        event_idx: 0,
        current_event: &log[0],
        planets: vec![],
    };
    let screen_x_center = screen_width() / 2.0;
    let screen_y_center = screen_height() / 2.0;


    loop {
        clear_background(DARKGRAY);
        let passed = get_time();
        if passed > state.step * (state.event_idx + 1) as f64 {
            state.event_idx += 1;
            state.current_event = &log[state.event_idx];
            state.event_handler();
        }
    
        for p in state.planets.iter_mut() {
            p.get_position();
            let [x, y] = p.position;
            draw_circle(screen_x_center + x - p.orbit.focus, screen_y_center + y, 3.0, RED);
            
        }
        draw_circle(screen_x_center, screen_y_center, 10.0, YELLOW);

        draw_text(&state.current_event.name(), 20.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}
