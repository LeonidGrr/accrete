function getSmallestMolecularWeight(m) {
    if (m < MOLECULAR_HYDROGEN)
	return ("H2");
    else if (m < HELIUM)
	return ("He");
    else if (m < METHANE)
	return ("CH4");
    else if (m < AMMONIA)
	return ("NH3");
    else if (m < WATER_VAPOR)
	return ("H2O");
    else if (m < NEON)
	return ("Ne");
    else if (m < MOLECULAR_NITROGEN)
	return ("N2");
    else if (m < CARBON_MONOXIDE)
	return ("CO");
    else if (m < NITRIC_OXIDE)
	return ("NO");
    else if (m < MOLECULAR_OXYGEN)
	return ("O2");
    else if (m < HYDROGEN_SULPHIDE)
	return ("H2S");
    else if (m < ARGON)
	return ("Ar");
    else if (m < CARBON_DIOXIDE)
	return ("CO2");
    else if (m < NITROUS_OXIDE)
	return ("N2O");
    else if (m < NITROGEN_DIOXIDE)
	return ("NO2");
    else if (m < OZONE)
	return ("O3");
    else if (m < SULPHUR_DIOXIDE)
	return ("SO2");
    else if (m < SULPHUR_TRIOXIDE)
	return ("SO3");
    else if (m < KRYPTON)
	return ("Kr");
    else if (m < XENON)
	return ("Xe");
    else
	return ("OTHER");
}

var systemToJSON = function(headPointer) {

    var jsonOut = {
	"star" : {
	    "mass" : stellar_mass_ratio,
	    "luminosity" : stellar_luminosity_ratio,
	    "mainSequenceLifetime" : (main_seq_life / 1.0E6),
	    "currentAge" : (age / 1.0E6),
	    "ecospherRadius" : r_ecosphere
	},
	"planets" : []
    };

    var node1 = headPointer;
    var counter = 1;

    while (node1 != NULL) {
	planetObject = {
	    "number" : counter,
	    "gasGiant" : node1.gas_giant,
	    "resonantPeriod" : node1.resonant_period,
	    "distanceFromPrimaryStar" : node1.a,
	    "eccentricity" : node1.e,
	    "mass" : node1.mass * EARTH_MASSES_PER_SOLAR_MASS,
	    "equatorialRadius" : node1.radius,
	    "density" : node1.density,
	    "escapeVelocity" : node1.escape_velocity / CM_PER_KM,
	    "smallestMolecularWeight" : getSmallestMolecularWeight(node1.molecule_weight),
	    "surfaceAcceleration" : node1.surface_accel,
	    "surfaceGravity" : node1.surface_grav,
	    "boilingPointOfWater" : (node1.boil_point - KELVIN_CELCIUS_DIFFERENCE),
	    "surfacePressure" : (node1.surface_pressure / 1000.0),
	    "greenhouseEffect" : node1.greenhouse_effect,
	    "surfaceTemperature" : (node1.surface_temp - KELVIN_CELCIUS_DIFFERENCE),
	    "hydrospherePercentage" : (node1.hydrosphere * 100),
	    "cloudCoverPercentage" : (node1.cloud_cover * 100),
	    "iceCoverPercentage" : (node1.ice_cover * 100),
	    "axialTilt" : node1.axial_tilt,
	    "albedo" : node1.albedo,
	    "lengthOfYear" : (node1.orbital_period / 365.25),
	    "lengthOfDay" : node1.day,

	};
	jsonOut.planets.push(planetObject);
	counter++;
	node1 = node1.next_planet;
    }

    return jsonOut;
};

var display_system = function() {
    
    /*
    var output = function(msg) {
	document.getElementById("output").innerHTML += msg;
    };
    
    var node1 ;
    var counter;
    output(sprintf("                         SYSTEM  CHARACTERISTICS\n"));
    output(sprintf("Mass of central star:          %6.3f solar masses\n", stellar_mass_ratio));
    output(sprintf("Luminosity of central star:    %6.3f (relative to the sun)\n", stellar_luminosity_ratio));
    output(sprintf("Total main sequence lifetime:  %6.0f million years\n", (main_seq_life / 1.0E6)));
    output(sprintf("Current age of stellar system: %6.0f million years\n", (age / 1.0E6)));
    output(sprintf("Radius of habitable ecosphere: %6.3f AU\n\n", r_ecosphere));
    node1 = planet_head;
    counter = 1;
    while (node1 != NULL) {
	output(sprintf("Planet #%d:\n", counter));
	if (node1.gas_giant) {
	    output(sprintf("Gas giant...\n"));
	}
	if (node1.resonant_period) {
	    output(sprintf("In resonant period with primary.\n"));
	}
	output(sprintf("   Distance from primary star (in A.U.): %7.3f\n", node1.a));
	output(sprintf("   Eccentricity of orbit:                %7.3f\n", node1.e));
	output(sprintf("   Mass (in Earth masses):               %7.3f\n", node1.mass * EARTH_MASSES_PER_SOLAR_MASS));
	output(sprintf("   Equatorial radius (in Km):            %7.1f\n", node1.radius));
	output(sprintf("   Density (in g/cc):                    %7.3f\n", node1.density));
	output(sprintf("   Escape Velocity (in km/sec):          %7.2f\n", node1.escape_velocity / CM_PER_KM));
	output(sprintf("   Smallest molecular weight retained:   %7.2f", node1.molecule_weight));
	output("   "+getSmallestMolecularWeight(node1.molecule_weight)+"\n");
	output(sprintf("   Surface acceleration (in cm/sec2):    %7.2f\n", node1.surface_accel));
	if (!(node1.gas_giant)) {
	    output(sprintf("   Surface Gravity (in Earth gees):      %7.2f\n", node1.surface_grav));
	    if (node1.boil_point > 0.1)
		output(sprintf("   Boiling point of water (celcius):     %7.1f\n", (node1.boil_point - KELVIN_CELCIUS_DIFFERENCE)));
	    if (node1.surface_pressure > 0.00001) {
		output(sprintf("   Surface Pressure (in atmospheres):    %7.3f", (node1.surface_pressure / 1000.0)));
		if (node1.greenhouse_effect)
		    output(sprintf("     RUNAWAY GREENHOUSE EFFECT\n"));
		else
		    output(sprintf("\n"));
	    }
	    output(sprintf("   Surface temperature (Celcius):        %7.2f\n", (node1.surface_temp - KELVIN_CELCIUS_DIFFERENCE)));
	    if (node1.hydrosphere > 0.01)
		output(sprintf("   Hydrosphere percentage: %6.2f\n", (node1.hydrosphere * 100)));
	    if (node1.cloud_cover > 0.01)
		output(sprintf("   Cloud cover percentage: %6.2f\n", (node1.cloud_cover * 100)));
	    if (node1.ice_cover > 0.01)
		output(sprintf("   Ice cover percentage:   %6.2f\n", (node1.ice_cover * 100)));
	}
	output(sprintf("   Axial tilt (in degrees):   %7d\n", node1.axial_tilt));
	output(sprintf("   Planetary albedo:          %7.3f\n", node1.albedo));
	output(sprintf("   Length of year (in years): %7.2f\n", (node1.orbital_period / 365.25)));
	output(sprintf("   Length of day (in hours):  %7.2f\n\n", node1.day));
	counter++;
	node1 = node1.next_planet;
    }
    */
};