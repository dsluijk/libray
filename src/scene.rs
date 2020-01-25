extern crate image;

use image::{ImageBuffer, Rgba};

/// The actual scene.
/// It keeps track of objects in the scene, and takes care of rendering the image.
pub struct Scene {
	buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
	width: u32,
	height: u32,
}

impl Scene {
	/// Create a new scene, with the width and height specified.
	pub fn new(width: u32, height: u32) -> Scene {
		let buffer = ImageBuffer::new(width, height);

		Scene {
			buffer,
			width,
			height,
		}
	}
}

impl Scene {
	/// Render the scene to a file.
	/// The path should have a .png or .jpg extension.
	pub fn render(&mut self, path: String) {
		for x in 0..self.width {
			for y in 0..self.height {
				let pixel = self.buffer.get_pixel_mut(x, y);
				*pixel = Rgba([
					100,
					(x * 255 / self.width) as u8,
					(y * 255 / self.height) as u8,
					255,
				]);
			}
		}

		self.buffer.save(path).unwrap();
	}
}
