use crate::audio::AudioHandler;
use crate::input::InputHandler;
use crate::graphics::{GraphicsHandler, TextureCache};

pub struct LayerData<'update> {
	pub audio: &'update mut AudioHandler,
	pub graphics: &'update mut GraphicsHandler,
	pub texture_cache: &'update mut TextureCache,
	pub input: &'update InputHandler,
}

pub trait Layer {
	fn init(&mut self, _: &mut LayerData) {}
	fn update(&mut self, _: &mut LayerData) {}
	fn render(&mut self, _: &mut LayerData) {}
}

pub struct EmptyLayer;
impl Layer for EmptyLayer {}