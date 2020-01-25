/// A color on the screen.
/// Used to color objects.
pub struct Color(u8, u8, u8);

impl Color {
	/// Create a new color, with the specified values.
	pub fn new(r: u8, g: u8, b: u8) -> Color {
		Color(r, g, b)
	}

	/// Get the red value in range 0 - 255.
	pub fn r(&self) -> u8 {
		self.0
	}

	/// Get the green value in range 0 - 255.
	pub fn g(&self) -> u8 {
		self.1
	}

	/// Get the blue value in range 0 - 255.
	pub fn b(&self) -> u8 {
		self.2
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_getters() {
		let color = Color::new(255, 42, 99);

		assert_eq!(color.r(), 255);
		assert_eq!(color.g(), 42);
		assert_eq!(color.b(), 99);
	}
}
