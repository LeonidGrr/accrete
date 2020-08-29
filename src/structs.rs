#[derive(Debug, Copy, Clone)]
pub struct DustBand {
    pub outer_edge: f64,
    pub inner_edge: f64,
    pub dust_present: bool,
    pub gas_present: bool,
}

impl DustBand {
    pub fn new(outer_edge: f64, inner_edge: f64, dust_present: bool, gas_present: bool) -> Self {
        Self {
            outer_edge,
            inner_edge,
            dust_present,
            gas_present,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Planetismal {
    pub a: f64, /* semi-major axis of the orbit (in AU)*/
    pub e: f64, /* eccentricity of the orbit	     */
    pub mass: f64, /* mass (in solar masses)	     */
    pub gas_giant: book, /* TRUE if the planet is a gas giant */
    pub orbit_zone: i32, /* the 'zone' of the planet          */
    pub radius: f64, /* equatorial radius (in km)	     */
    pub density: f64, /* density (in g/cc)		     */
    pub orbital_period: f64, /* length of the local year (days)   */
    pub day: f64, /* length of the local day (hours)   */
    pub resonant_period: f64, /* TRUE if in resonant rotation   */
    pub axial_tilt: f64, /* units of degrees		     */
    pub escape_velocity: f64, /* units of cm/sec		     */
    pub surface_accel: f64, /* units of cm/sec2		     */
    pub surface_grav: f64, /* units of Earth gravities	     */
    pub rms_velocity: f64, /* units of cm/sec		     */
    pub molecule_weight: f64, /* smallest molecular weight retained*/
    pub volatile_gas_inventory: f64,
    pub surface_pressure: f64, /* units of millibars (mb)	     */
    pub greenhouse_effect: f64, /* runaway greenhouse effect?	*/
    pub boil_point: f64, /* the boiling point of water (Kelvin)*/
    pub albedo: f64, /* albedo of the planet		     */
    pub surface_temp: f64, /* surface temperature in Kelvin     */
    pub hydrosphere: f64, /* fraction of surface covered	     */
    pub cloud_cover: f64, /* fraction of surface covered	     */
    pub ice_cover: f64, /* fraction of surface covered	     */
    pub moons: Vec<Planetismal>,
}
