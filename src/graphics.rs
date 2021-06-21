use polystrip::{RenderSize, Renderer, Texture, WindowTarget};
use polystrip::gon::{ColoredShape, GonFrame, GonPipeline, TexturedShape};
use polystrip::math::{Color, Matrix4, Rect};
use polystrip::pixel::PixelTranslator;

use std::collections::HashMap;

use winit::window::Window;

const RECT_INDICES: [[u16; 3]; 2] = [[0, 3, 1], [1, 3, 2]];

type TextureID = usize;

pub struct GraphicsHandler {
	renderer: WindowTarget,
	pipeline: GonPipeline,
	pixel_translator: PixelTranslator,

	texture_cache: HashMap<TextureID, Texture>,
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

			texture_cache: HashMap::new(),
		}
	}

	pub fn canvas(&mut self) -> Canvas {
		Canvas::new(self.renderer.next_frame().render_with(&mut self.pipeline), &self.pixel_translator)
	}

	pub fn get_texture(&mut self, id: TextureID) -> Option<&Texture> {
		self.texture_cache.get(&id)
	}

	pub fn load_texture(&mut self, texture: Texture) -> TextureID {
		self.texture_cache.insert(self.texture_cache.len(), texture);
		self.texture_cache.len() - 1
	}
}

pub struct Canvas<'canvas> {
	frame: GonFrame<'canvas>,
	pixel_translator: &'canvas PixelTranslator,
}

impl<'canvas> Canvas<'canvas> {
	fn new(frame: GonFrame<'canvas>, pixel_translator: &'canvas PixelTranslator) -> Self {
		Self {
			frame,
			pixel_translator,
		}
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
