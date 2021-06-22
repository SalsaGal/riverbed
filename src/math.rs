pub use polystrip::math::*;

#[derive(Clone, Copy, Debug)]
pub struct Point<T> where T: Clone + Copy {
	pub x: T,
	pub y: T,
}

impl<T> Point<T> where T: Clone + Copy {
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

	pub fn origin() -> Self where T: Default {
		Self {
			x: T::default(),
			y: T::default(),
		}
	}
}
