pub use polystrip::math::*;

pub struct Point<T> {
	pub x: T,
	pub y: T,
}

impl<T> Point<T> {
	pub fn new(x: T, y: T) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn square(w: T) -> Self where T: Copy {
		Self {
			x: w,
			y: w,
		}
	}
}
