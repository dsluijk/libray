use std::ops::Add;

/// A position vector.
/// Initialized with the x, y and z value.
pub struct Position(f64, f64, f64);

impl Add<Position> for Position {
	type Output = Position;

	/// Add two vectors together.
	fn add(self, other: Position) -> Position {
		Position(self.0 + other.0, self.1 + other.1, self.2 + other.2)
	}
}

impl Position {
	/// Create a new position.
	pub fn new(x: f64, y: f64, z: f64) -> Position {
		Position(x, y, z)
	}

	/// Get the x value of the position.
	pub fn x(&self) -> f64 {
		self.0
	}

	/// Get the y value of the position.
	pub fn y(&self) -> f64 {
		self.1
	}

	/// Get the z value of the position.
	pub fn z(&self) -> f64 {
		self.2
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_getters() {
		let pos = Position::new(1.0, 42.0, 55.0);

		assert_eq!(pos.x() - pos.0 < 0.001, true);
		assert_eq!(pos.y() - pos.1 < 0.001, true);
		assert_eq!(pos.z() - pos.2 < 0.001, true);
	}

	#[test]
	fn test_addition() {
		let pos1 = Position::new(1.0, 42.0, 55.0);
		let pos2 = Position::new(4.0, 76.0, 11.0);
		let posRes = pos1 + pos2;

		assert_eq!(posRes.x() - 5.0 < 0.001, true);
		assert_eq!(posRes.y() - 118.0 < 0.001, true);
		assert_eq!(posRes.z() - 66.0 < 0.001, true);
	}
}
