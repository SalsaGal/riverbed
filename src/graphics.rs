use polystrip::{RenderSize, Renderer, Texture, WindowTarget};
use polystrip::gon::{ColoredShape, GonFrame, GonPipeline, TexturedShape};
use polystrip::math::{Color, Matrix4, Rect};
use polystrip::pixel::PixelTranslator;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use winit::window::Window;

const RECT_INDICES: [[u16; 3]; 2] = [[0, 3, 1], [1, 3, 2]];

pub type TextureID = usize;

pub struct TextureCache {
	cache: HashMap<TextureID, Texture>,
}

impl TextureCache {
	pub fn new() -> Self {
		Self {
			cache: HashMap::new(),
		}
	}

	pub fn insert(&mut self, id: TextureID, texture: Texture) -> Option<Texture> {
		self.cache.insert(id, texture)
	}

	pub fn get(&self, id: TextureID) -> Option<&Texture> {
		self.cache.get(&id)
	}

	pub fn push(&mut self, texture: Texture) -> TextureID {
		self.cache.insert(self.cache.len(), texture);
		self.cache.len() - 1
	}

	pub fn clear(&mut self) {
		self.cache.clear()
	}
}

pub struct GraphicsHandler {
	renderer: WindowTarget,
	pipeline: GonPipeline,
	pixel_translator: PixelTranslator,
}

impl GraphicsHandler {
	pub fn new(window: &Window) -> Self {
		let window_size = RenderSize::new(window.inner_size().width, window.inner_size().height).wrap();
		let renderer = WindowTarget::new(Renderer::new().wrap(), window, &window_size, 3);
		let pipeline = GonPipeline::new(&renderer, &renderer);
		let pixel_translator = renderer.pixel_translator();

		Self {
			renderer,
			pipeline,
			pixel_translator,
		}
	}

	pub fn canvas(&mut self) -> Canvas {
		let mut frame = self.renderer.next_frame().render_with(&mut self.pipeline); 
		frame.clear(Color::BLACK);

		Canvas {
			frame,
			pixel_translator: &self.pixel_translator,
		}
	}

	pub fn texture_from_file(&mut self, path: &str) -> Result<Texture, ()> {
		if let Ok(mut file) = File::open(path) {
			let mut bytes = Vec::new();
			file.read_to_end(&mut bytes).unwrap();
			Ok(self.texture_from_data(bytes))
		} else {
			eprintln!("Can't find file {}", path);
			Err(())
		}
	}

	pub fn texture_from_data(&mut self, data: Vec<u8>) -> Texture {
		let img = image::load_from_memory(&data[..]).unwrap().to_rgba8();
		Texture::new_from_rgba(&self.renderer, &*img, img.dimensions())
	}
}

pub struct Canvas<'canvas> {
	frame: GonFrame<'canvas>,
	pixel_translator: &'canvas PixelTranslator,
}

impl<'canvas> Canvas<'canvas> {
	pub fn clear(&mut self, color: Color) {
		self.frame.clear(color);
	}

	pub fn fill_rect(&mut self, bounds: Rect, color: Color) {
		self.frame.draw_colored(
			&ColoredShape {
				vertices: self.pixel_translator.colored_rect(bounds, color)[..].into(),
				indices: RECT_INDICES[..].into(),
			},
			&[Matrix4::identity()],
		);
	}

	pub fn draw_texture(&mut self, bounds: Rect, texture: &'canvas Texture) {
		self.frame.draw_textured(
			&TexturedShape {
				vertices: self.pixel_translator.textured_rect(bounds)[..].into(),
				indices: RECT_INDICES[..].into()
			},
			&[(texture, &[Matrix4::identity()])],
		);
	}
}
