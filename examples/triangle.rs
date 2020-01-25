extern crate libray;

use libray::{Color, Position};

fn main() {
	let mut scene = libray::Scene::new(800, 800);
	scene.render(String::from("out.png"));

	let _color = Color::new(200, 0, 0);
	let _pos = Position::new(1.0, 1.0, 1.0);
}
