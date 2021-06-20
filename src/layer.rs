use crate::graphics::Canvas;

pub trait Layer {
	fn update(&mut self) {}
	fn render(&mut self, _: &mut Canvas) {}
}

pub struct EmptyLayer;
impl Layer for EmptyLayer {}