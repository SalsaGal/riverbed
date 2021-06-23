pub use polystrip::math::*;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Clone, Copy, Debug)]
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

	pub fn square(w: T) -> Self where T: Clone + Copy {
		Self {
			x: w,
			y: w,
		}
	}
}

impl<T> Default for Point<T> where T: Default {
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
		}
	}
}

impl<T> Add for Point<T> where T: Add<Output = T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> Sub for Point<T> where T: Sub<Output = T> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> Mul for Point<T> where T: Mul<Output = T> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl<T> Div for Point<T> where T: Div<Output = T> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}

impl<T> AddAssign for Point<T> where T: AddAssign {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> SubAssign for Point<T> where T: SubAssign {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T> MulAssign for Point<T> where T: MulAssign {
	fn mul_assign(&mut self, rhs: Self) {
		self.x *= rhs.x;
		self.y *= rhs.y;
	}
}

impl<T> DivAssign for Point<T> where T: DivAssign {
	fn div_assign(&mut self, rhs: Self) {
		self.x /= rhs.x;
		self.y /= rhs.y;
	}
}
