
//use rand::prelude::*;
extern crate macroquad;
use macroquad::prelude::*;
// use macroquad::shapes::* as macroquadshapes;
extern crate num_traits;
use num_traits::pow::pow as Pow;
//
extern crate console_error_panic_hook;
use std::panic;
//

//
fn RandomNumberBt0and1() -> f64 {panic!(); 0.5} // Not used yet
fn LocalDrawCircle(a: f64, b: f64, c: f64, d: macroquad::prelude::Color) { // called this because it's the local version
	macroquad::prelude::draw_circle(a as f32, b as f32, c as f32, d)
}

// const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = (6.6743015 / (10 ^ 11)); // Commented out to make physics not require huge values for mass to do gravity
const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 1.0;
const COLOUR_BLACK: macroquad::color::Color = macroquad::color::Color::new(0.00, 0.00, 0.00, 1.00);
const COLOUR_WHITE: macroquad::color::Color = macroquad::color::Color::new(1.00, 1.00, 1.00, 1.00);


//
#[derive(Debug, Clone)]
struct PlanetaryBody {
	mass: f64,
	radius: f64,
	location: [f64; 2],
	velocity: [f64; 2]
}

// The impl block defines properties of the type specified. Here, the type specified is PlanetaryBody. 
impl PlanetaryBody {  
	fn SelfAdjustVelocityForGravityToOtherObject(mut self, body_2_r: &PlanetaryBody, delta_time: f64) -> PlanetaryBody {
		let x_displacement: f64 = body_2_r.location[0] - self.location[0];
		let y_displacement: f64 = body_2_r.location[1] - self.location[1];
		let distance: f64 = f64::sqrt(Pow(x_displacement, 2) + Pow(y_displacement, 2));
		let force: f64 = UNIVERSAL_GRAVITATIONAL_CONSTANT * self.mass * body_2_r.mass / Pow(distance, 2);
		let vectors: [[f64; 2]; 2] = [[x_displacement / distance, y_displacement / distance], [0.0 - x_displacement / distance, 0.0 - y_displacement / distance]];
		self.velocity[0] += delta_time * force * vectors[0][0] / self.mass;
		self.velocity[1] += delta_time * force * vectors[0][1] / self.mass;
		self
	}
	fn PairwiseAdjustVelocityForGravity(body_1: &mut PlanetaryBody, body_2: &mut PlanetaryBody, delta_time: f64) {
		let x_displacement: f64 = body_2.location[0] - body_1.location[0];
		let y_displacement: f64 = body_2.location[1] - body_1.location[1];
		let distance: f64 = f64::sqrt(Pow((x_displacement), 2) + Pow((y_displacement), 2));
		// if distance = (0 as f64) {break} // this should throw an error somehow
		let force: f64 = UNIVERSAL_GRAVITATIONAL_CONSTANT * body_1.mass * body_2.mass / Pow(distance, 2);
		let vectors: [[f64; 2]; 2] = [[x_displacement / distance, y_displacement / distance], [0.0 - x_displacement / distance, 0.0 - y_displacement / distance]];
		body_1.velocity[0] = body_1.velocity[0] + (delta_time * force * vectors[0][0] / body_1.mass);
		body_1.velocity[1] = body_1.velocity[1] + (delta_time * force * vectors[0][1] / body_1.mass);
		body_2.velocity[0] = body_2.velocity[0] + (delta_time * force * vectors[1][0] / body_2.mass);
		body_2.velocity[1] = body_2.velocity[1] + (delta_time * force * vectors[1][1] / body_2.mass);
	}
	fn SelfAdjustLocationForVelocity(self: &mut Self, delta_time: f64) {
		self.location[0] = self.location[0] + self.velocity[0] * delta_time;
		self.location[1] = self.location[1] + self.velocity[1] * delta_time;
	}
	fn PairwiseFindDistanceBetween(body_1: &PlanetaryBody, body_2: &PlanetaryBody) -> f64 {f64::sqrt(Pow(body_1.location[0] - body_2.location[0], 2) + Pow(body_1.location[1] - body_2.location[1], 2))}
	fn PairwiseCheckForCollision(body_1: &PlanetaryBody, body_2: &PlanetaryBody) -> bool {body_1.radius + body_2.radius < PlanetaryBody::PairwiseFindDistanceBetween(body_1, body_2)}
} // this is the end of the impl block

fn PhysicsTick(mut planetary_bodies: Vec::<PlanetaryBody>, delta_time: f64) -> Vec::<PlanetaryBody> {
	//'collision_checks: loop {break 'collision_checks;} // check and handle collisions. break added temporarily, commented out for skipping initially
	let number_of_bodies: usize = planetary_bodies.len();
	for index in 0..(number_of_bodies - 1) {
		planetary_bodies[index].SelfAdjustLocationForVelocity(delta_time)
	}
	for first_index in 1..(number_of_bodies-1) {
		for second_index in first_index..number_of_bodies {
			planetary_bodies[first_index] = planetary_bodies[first_index].clone().SelfAdjustVelocityForGravityToOtherObject(&planetary_bodies[second_index], delta_time);
			planetary_bodies[second_index] = planetary_bodies[second_index].clone().SelfAdjustVelocityForGravityToOtherObject(&planetary_bodies[first_index], delta_time);
		}; 
	};
	return planetary_bodies
}

fn RenderBodies(planetary_bodies_r: &Vec<PlanetaryBody>, view_attributes: [f64; 3]) {
	for item in planetary_bodies_r {
		LocalDrawCircle(item.location[0] * view_attributes[2] + view_attributes[0], item.location[1] * view_attributes[2] + view_attributes[1], item.radius * view_attributes[2], COLOUR_BLACK)
	}
}

//

//

//

#[macroquad::main("Assignment-Gravity-Sim")]
async fn main() {  // This is the function that is normally set to immediately execute on starting the program. 
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	
	//let mut fonts = Fonts::default();
	//let FONT_SPECTRAL_LIGHT: macroquad::text::Font = load_ttf_font("./fonts/Spectral-Light.ttf").await.unwrap(); // Claims that .await is only allowed inside async function, so i moved it here. However, it's still complaining.
	//let FONT_SPECTRAL_LIGHT_ITALIC: macroquad::text::Font = load_ttf_font("./fonts/Spectral-LightItalic.ttf").await.unwrap();
	let mut view_attributes: [f64; 3] = [(macroquad::prelude::screen_width() as f64) / 2.0, (macroquad::prelude::screen_height() as f64) / 2.0, 1.0];
	let mut planetary_bodies: Vec<PlanetaryBody> = Vec::<PlanetaryBody>::with_capacity(64);
	planetary_bodies.push(PlanetaryBody {mass: 1.0, radius: (macroquad::prelude::screen_height() as f64) / 20.0, velocity: [0.0, 5.0], location: [{macroquad::prelude::screen_width() * 0.75} as f64, 0.0]});
	planetary_bodies.push(PlanetaryBody {mass: 1.0, radius: (macroquad::prelude::screen_height() as f64) / 20.0, velocity: [0.0, -5.0], location: [{macroquad::prelude::screen_width() * 0.25} as f64, 0.0]});
	'main_cycle: loop {
		clear_background(COLOUR_WHITE);
		RenderBodies(&planetary_bodies, view_attributes);
		/*{
			let font = FONT_SPECTRAL_LIGHT.clone();
			macroquad::text::draw_text("hello", view_attributes[0] as f32, view_attributes[1] as f32, 20.0, COLOUR_BLACK);
			macroquad::text::draw_text("hello", 0.0, 0.0, 20.0, COLOUR_BLACK);
		}*/
		//fonts.draw_text("hello", view_attributes[0] as f32, view_attributes[1] as f32, 20.0, COLOUR_BLACK);
		//fonts.draw_text("hello", 0.0, 0.0, 20.0, COLOUR_BLACK);
		println!("{:#?}", &planetary_bodies);
		let delta_time = get_frame_time();
		planetary_bodies = PhysicsTick(planetary_bodies, delta_time as f64); //changed to just pass the bodies back and forth to get around mutable reference issues
				next_frame().await
	}
	
}
