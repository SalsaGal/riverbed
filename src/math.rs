pub struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	pub fn new(x: T, y: T) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn square(w: T) -> Self {
		Self {
			x: w,
			y: w,
		}
	}
}
