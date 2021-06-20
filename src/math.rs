pub struct Vector2<T> {
	x: T,
	y: T,
}

impl<T> Vector2<T> {
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
