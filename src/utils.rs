// function random_number(inner, outer) {
//     var delta = Math.abs(outer - inner);
//     if (inner < outer) {
// 	return (inner + delta * randomTool.genrand_real3());
//     } else {
// 	return (outer + delta * randomTool.genrand_real3());
//     }
// };

// /*----------------------------------------------------------------------*/
// /* This function returns a value within a certain variation of the */
// /* exact value given it in 'value'. */
// /*----------------------------------------------------------------------*/
// function about(value, variation) {
//     var inner = value - variation;
//     return (inner + 2.0 * variation * randomTool.genrand_real3());
// };

// function random_eccentricity() {
//     return (1.0 - Math.pow((randomTool.genrand_real3()), ECCENTRICITY_COEFF));
// };