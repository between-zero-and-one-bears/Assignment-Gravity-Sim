use std::time::{SystemTime, UNIX_EPOCH};
extern crate num_traits;
use num_traits::pow::pow as Pow;
extern crate macroquad;
use macroquad::prelude::*;
//extern crate console_error_panic_hook; // causes gray screen
//use std::panic;

fn nanoseconds_get() -> u32 {
	if let Ok(duration) = UNIX_EPOCH.elapsed() {
		return duration.subsec_nanos()
	} else {panic!()}
}
fn temporal_random_f32() -> f32 {(nanoseconds_get() as f32) / (1000000000.000 as f32)}



fn local_draw_circle(a: f64, b: f64, c: f64, d: macroquad::prelude::Color) { // called this because it's the local version
	macroquad::prelude::draw_circle(a as f32, b as f32, c as f32, d)
}

// const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = (6.6743015 / (10 ^ 11)); // Commented out to make physics not require huge values for mass to do gravity
const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 100.0;

#[derive(Debug, Clone)]
struct PlanetaryBody {
	mass: f64,
	radius: f64,
	location: [f64; 2],
	velocity: [f64; 2],
	colour: macroquad::prelude::Color
}

// The impl block defines properties of the type specified. Here, the type specified is PlanetaryBody. 
impl PlanetaryBody {  
	fn self_adjust_velocity_for_gravity_to_other_object(mut self, body_2_r: &PlanetaryBody, delta_time: f64) -> PlanetaryBody {
		let x_displacement: f64 = body_2_r.location[0] - self.location[0];
		let y_displacement: f64 = body_2_r.location[1] - self.location[1];
		let distance: f64 = f64::sqrt((x_displacement * x_displacement) + (y_displacement * y_displacement));
		let acceleration: f64 = UNIVERSAL_GRAVITATIONAL_CONSTANT * body_2_r.mass / (distance * distance);
		self.velocity[0] += delta_time * acceleration * x_displacement / distance;
		self.velocity[1] += delta_time * acceleration * y_displacement / distance;
		return self
	}
	fn self_adjust_location_for_velocity(mut self, delta_time: f64) -> PlanetaryBody {
		self.location[0] += self.velocity[0] * delta_time;
		self.location[1] += self.velocity[1] * delta_time;
		return self
	}
	//fn PairwiseCheckForCollision(body_1: &PlanetaryBody, body_2: &PlanetaryBody) -> bool {body_1.radius + body_2.radius < f64::sqrt(Pow(body_2.location[0] - body_1.location[0], 2) + Pow(body_2.location[1] - body_1.location[1], 2))}
} // this is the end of the impl block

fn physics_tick(mut planetary_bodies: Vec::<PlanetaryBody>, delta_time: f64) -> Vec::<PlanetaryBody> {
	//'collision_checks: loop {break 'collision_checks;} // check and handle collisions. break added temporarily, commented out for skipping initially
	let number_of_bodies: usize = planetary_bodies.len();
	for index in 0..number_of_bodies {
		planetary_bodies[index] = planetary_bodies[index].clone().self_adjust_location_for_velocity(delta_time)
	}
	for first_index in 0..(number_of_bodies - 1) {
		for second_index in (first_index + 1)..number_of_bodies {
			planetary_bodies[first_index] = planetary_bodies[first_index].clone().self_adjust_velocity_for_gravity_to_other_object(planetary_bodies.get(second_index).unwrap(), delta_time);
			planetary_bodies[second_index] = planetary_bodies[second_index].clone().self_adjust_velocity_for_gravity_to_other_object(planetary_bodies.get(first_index).unwrap(), delta_time);
		}; 
	};
	return planetary_bodies
}

fn render_bodies(planetary_bodies_r: &Vec<PlanetaryBody>, view_attributes: [f64; 3]) {
	for item in planetary_bodies_r {
		local_draw_circle(item.location[0] * view_attributes[2] + view_attributes[0], item.location[1] * view_attributes[2] + view_attributes[1], item.radius * view_attributes[2], item.colour)
	}
}

//

//

//

#[macroquad::main("Assignment-Gravity-Sim")]
async fn main() {  // This is the function that is normally set to immediately execute on starting the program. 
	//panic::set_hook(Box::new(console_error_panic_hook::hook));
	let mut view_attributes: [f64; 3] = [(macroquad::prelude::screen_width() as f64) / 2.0, (macroquad::prelude::screen_height() as f64) / 2.0, 1.0];
	let mut planetary_bodies: Vec<PlanetaryBody> = Vec::<PlanetaryBody>::with_capacity(64);
	//planetary_bodies.push(PlanetaryBody {mass: 100.0, radius: (macroquad::prelude::screen_height() as f64) / 20.0, velocity: [0.0, 5.0], location: [{0.0 - {macroquad::prelude::screen_width() * 0.0625}} as f64, 0.0], colour: macroquad::prelude::RED});
	//planetary_bodies.push(PlanetaryBody {mass: 5.0, radius: (macroquad::prelude::screen_height() as f64) / 20.0, velocity: [0.0, -5.0], location: [{macroquad::prelude::screen_width() * 0.0625} as f64, 0.0], colour: macroquad::prelude::BLUE});
	planetary_bodies.push(PlanetaryBody {mass: 10000.0, radius: (macroquad::prelude::screen_height() as f64) / 20.0, velocity: [0.0, 0.0], location: [{0.0 - {macroquad::prelude::screen_width() * 0.0625}} as f64, 0.0], colour: macroquad::prelude::RED});
	planetary_bodies.push(PlanetaryBody {mass: 5.0, radius: (macroquad::prelude::screen_height() as f64) / 50.0, velocity: [5.0, 50.0], location: [{macroquad::prelude::screen_width() * 0.0625} as f64, 0.0], colour: macroquad::prelude::BLUE});
	planetary_bodies.push(PlanetaryBody {mass: 5.0, radius: (macroquad::prelude::screen_height() as f64) / 50.0, velocity: [10.0, -40.0], location: [{macroquad::prelude::screen_width() * 0.0725} as f64, 100.0], colour: macroquad::prelude::GREEN});
	planetary_bodies.push(PlanetaryBody {mass: 2.0, radius: (macroquad::prelude::screen_height() as f64) / 80.0, velocity: [10.0, -40.0], location: [{{macroquad::prelude::screen_width() * 0.0425}} as f64, -100.0], colour: macroquad::prelude::YELLOW});
	//temporal_random_f32();
	loop {
		clear_background(macroquad::prelude::WHITE);
		render_bodies(&planetary_bodies, view_attributes);
		let delta_time = get_frame_time();
		/*{
			let font = FONT_SPECTRAL_LIGHT.clone();
			macroquad::text::draw_text("hello", view_attributes[0] as f32, view_attributes[1] as f32, 20.0, macroquad::prelude::BLACK);
			macroquad::text::draw_text("hello", 0.0, 0.0, 20.0, macroquad::prelude::BLACK);
		}*/
		//fonts.draw_text("hello", view_attributes[0] as f32, view_attributes[1] as f32, 20.0, macroquad::prelude::BLACK);
		//fonts.draw_text("hello", 0.0, 0.0, 20.0, macroquad::prelude::BLACK);
		//println!("{:#?}", &planetary_bodies);
		if is_mouse_button_pressed(MouseButton::Left) {
			let (mouse_x,mouse_y) = mouse_position();
			planetary_bodies.push(PlanetaryBody {mass: 2.0, radius: (macroquad::prelude::screen_height() as f64) / 100.0, velocity: [10.0, -40.0], location: [mouse_x as f64 - view_attributes[0],mouse_y as f64 - view_attributes[1]], colour: macroquad::prelude::RED});

		}                  //THIS SECTION IS NOT THE ISSUE
		planetary_bodies = physics_tick(planetary_bodies, delta_time as f64); //changed to just pass the bodies back and forth to get around mutable reference issues
		next_frame().await
	}
	
}
