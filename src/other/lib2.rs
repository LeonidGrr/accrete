/*  
These are all of the global variables used during accretion: 
anum
planet_head
stellar_mass_ratio
stellar_luminosity_ratio
main_seq_life
double age
r_ecosphere
r_greenhouse
radians_per_rotation
spin_resonance
randomTool
 */

 var anum;
 var planet_head;
 var stellar_mass_ratio;
 var stellar_luminosity_ratio;
 var main_seq_life;
 var age, r_ecosphere;
 var r_greenhouse;
 var radians_per_rotation;
 var spin_resonance;
 var randomTool;
 
 
 function generate_stellar_system() {
     var planet;
     radians_per_rotation = 2.0 * PI;
     stellar_mass_ratio = random_number(0.6, 1.3);
     stellar_luminosity_ratio = luminosity(stellar_mass_ratio);
 
     planet = distribute_planetary_masses(stellar_mass_ratio, stellar_luminosity_ratio, 0.0, stellar_dust_limit(stellar_mass_ratio));
 
     main_seq_life = 1.0E10 * (stellar_mass_ratio / stellar_luminosity_ratio);
     if ((main_seq_life >= 6.0E9))
     age = random_number(1.0E9, 6.0E9);
     else
     age = random_number(1.0E9, main_seq_life);
     r_ecosphere = Math.sqrt(stellar_luminosity_ratio);
     r_greenhouse = r_ecosphere * GREENHOUSE_EFFECT_CONST;
 
     while (planet != NULL) {
     planet.orbit_zone = orbital_zone(planet.a);
     if (planet.gas_giant) {
         planet.density = empirical_density(planet.mass, planet.a, planet.gas_giant);
         planet.radius = volume_radius(planet.mass, planet.density);
     } else {
         planet.radius = kothari_radius(planet.mass, planet.a, planet.gas_giant, planet.orbit_zone);
         planet.density = volume_density(planet.mass, planet.radius);
     }
     planet.orbital_period = period(planet.a, planet.mass, stellar_mass_ratio);
     planet.day = day_length(planet.mass, planet.radius, planet.orbital_period, planet.e, planet.gas_giant);
     planet.resonant_period = spin_resonance;
     planet.axial_tilt = inclination(planet.a);
     planet.escape_velocity = escape_vel(planet.mass, planet.radius);
     planet.surface_accel = acceleration(planet.mass, planet.radius);
     planet.rms_velocity = rms_vel(MOLECULAR_NITROGEN, planet.a);
     planet.molecule_weight = molecule_limit(planet.a, planet.mass, planet.radius);
     if ((planet.gas_giant)) {
         planet.surface_grav = INCREDIBLY_LARGE_NUMBER;
         planet.greenhouse_effect = FALSE;
         planet.volatile_gas_inventory = INCREDIBLY_LARGE_NUMBER;
         planet.surface_pressure = INCREDIBLY_LARGE_NUMBER;
         planet.boil_point = INCREDIBLY_LARGE_NUMBER;
         planet.hydrosphere = INCREDIBLY_LARGE_NUMBER;
         planet.albedo = about(GAS_GIANT_ALBEDO, 0.1);
         planet.surface_temp = INCREDIBLY_LARGE_NUMBER;
     } else {
         planet.surface_grav = gravity(planet.surface_accel);
         planet.greenhouse_effect = greenhouse(planet.orbit_zone, planet.a, r_greenhouse);
         planet.volatile_gas_inventory = vol_inventory(planet.mass, planet.escape_velocity, planet.rms_velocity, stellar_mass_ratio, planet.orbit_zone, planet.greenhouse_effect);
         planet.surface_pressure = pressure(planet.volatile_gas_inventory, planet.radius, planet.surface_grav);
         if ((planet.surface_pressure == 0.0))
         planet.boil_point = 0.0;
         else
         planet.boil_point = boiling_point(planet.surface_pressure);
         iterate_surface_temp(planet);
     }
     planet = planet.next_planet;
     }
 }
 
 function main() {
     randomTool = new MersenneTwister(1);
     generate_stellar_system();
     display_system();
 }