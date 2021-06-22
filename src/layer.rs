use crate::input::InputHandler;
use crate::graphics::{GraphicsHandler, TextureCache};

pub trait Layer {
	fn update(&mut self, _: &InputHandler) {}
	fn render(&mut self, _: &mut GraphicsHandler, _: &mut TextureCache) {}
}

pub struct EmptyLayer;
impl Layer for EmptyLayer {}